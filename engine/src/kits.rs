//! Loading of the three kits (06 §8.4) from the kits folder (05 §8).
//!
//! For each kit, `kit.json` gives the file and the choke group of each row; its WAV files are
//! read into memory as mono `f32`. Names are for the interface and are not read here.

use std::fs;
use std::path::Path;

use hound::{SampleFormat, WavReader};
use serde::Deserialize;

use crate::constants::{KIT_ROWS, SAMPLE_RATE};
use crate::error::EngineError;

/// Kit identifiers, in the order of their index (06 §5.3).
pub const KIT_IDS: [&str; 3] = ["acoustic", "electronic", "percussion"];

/// Role of each row of every kit, in row order (05 §8.1).
pub const KIT_ROLES: [&str; KIT_ROWS] = [
    "low",
    "snare",
    "clap",
    "hatClosed",
    "hatOpen",
    "tom",
    "accent",
    "extra",
];

/// Longest kit sound: 4 seconds (05 §8.3).
pub const MAX_KIT_SOUND_FRAMES: usize = 4 * SAMPLE_RATE as usize;

/// A 16-bit sample `v` is read as `v / 32768`, so that it lies in [−1, 1).
const INT16_SCALE: f32 = 32_768.0;

/// Index of a kit in [`KIT_IDS`].
pub fn kit_index(id: &str) -> Option<usize> {
    KIT_IDS.iter().position(|kit| *kit == id)
}

/// The three kits, in the order of [`KIT_IDS`].
#[derive(Debug, Clone, PartialEq)]
pub struct Kits {
    pub kits: [Kit; 3],
}

/// One kit: a sound per row, in row order.
#[derive(Debug, Clone, PartialEq)]
pub struct Kit {
    pub sounds: [KitSound; KIT_ROWS],
}

/// One sound of a kit.
#[derive(Debug, Clone, PartialEq)]
pub struct KitSound {
    /// Mono samples at `SAMPLE_RATE`.
    pub samples: Vec<f32>,
    /// 0 without a choke group; otherwise a number greater than 0, the same for every sound of
    /// the kit with that group name.
    pub choke_group: u8,
}

/// The parts of `kit.json` that the engine uses (05 §8.3).
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct KitFile {
    sounds: Vec<KitFileSound>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct KitFileSound {
    role: String,
    file: String,
    choke_group: Option<String>,
}

/// Loads the three kits from `kits_dir` (06 §8.4). Fails if a kit is missing or incomplete, or
/// if one of its sounds is not a mono WAV at 48 000 Hz, in 16 bits or in 32-bit floating point,
/// of at most 4 seconds.
pub fn load_kits(kits_dir: &Path) -> Result<Kits, EngineError> {
    let [acoustic, electronic, percussion] = KIT_IDS.map(|id| load_kit(&kits_dir.join(id)));
    Ok(Kits {
        kits: [acoustic?, electronic?, percussion?],
    })
}

/// Loads the kit in `dir`.
fn load_kit(dir: &Path) -> Result<Kit, EngineError> {
    let path = dir.join("kit.json");
    let text = fs::read_to_string(&path).map_err(|error| EngineError::from_io(&path, &error))?;
    let kit: KitFile = serde_json::from_str(&text)
        .map_err(|error| unsupported(&path, &format!("not a kit: {error}")))?;
    let roles: Vec<&str> = kit.sounds.iter().map(|sound| sound.role.as_str()).collect();
    if roles != KIT_ROLES {
        return Err(unsupported(
            &path,
            &format!("the roles are {roles:?}, not {KIT_ROLES:?}"),
        ));
    }

    let mut group_names: Vec<&str> = Vec::new();
    let mut sounds = Vec::with_capacity(KIT_ROWS);
    for sound in &kit.sounds {
        let choke_group = match sound.choke_group.as_deref() {
            None => 0,
            Some(name) => {
                let index = group_names.iter().position(|group| *group == name);
                let index = index.unwrap_or_else(|| {
                    group_names.push(name);
                    group_names.len() - 1
                });
                // There are at most KIT_ROWS groups, so the number fits in a u8.
                u8::try_from(index + 1).unwrap_or(u8::MAX)
            }
        };
        sounds.push(KitSound {
            samples: read_sound(&dir.join(&sound.file))?,
            choke_group,
        });
    }
    let sounds = sounds
        .try_into()
        .map_err(|_| unsupported(&path, "the kit does not have one sound per row"))?;
    Ok(Kit { sounds })
}

/// Reads a kit sound: mono, 48 000 Hz, 16 bits or 32-bit floating point, at most 4 seconds.
fn read_sound(path: &Path) -> Result<Vec<f32>, EngineError> {
    let mut reader = WavReader::open(path).map_err(|error| wav_error(path, error))?;
    let spec = reader.spec();
    if spec.channels != 1 || spec.sample_rate != SAMPLE_RATE {
        return Err(unsupported(
            path,
            &format!(
                "{} channels at {} Hz, not mono at {SAMPLE_RATE} Hz",
                spec.channels, spec.sample_rate
            ),
        ));
    }
    let frames = usize::try_from(reader.duration()).unwrap_or(usize::MAX);
    if frames > MAX_KIT_SOUND_FRAMES {
        return Err(unsupported(
            path,
            &format!("{frames} frames, more than {MAX_KIT_SOUND_FRAMES}"),
        ));
    }
    let samples: Result<Vec<f32>, hound::Error> = match (spec.sample_format, spec.bits_per_sample) {
        (SampleFormat::Int, 16) => reader
            .samples::<i16>()
            .map(|sample| sample.map(|value| f32::from(value) / INT16_SCALE))
            .collect(),
        (SampleFormat::Float, 32) => reader.samples::<f32>().collect(),
        (format, bits) => {
            return Err(unsupported(
                path,
                &format!("{bits}-bit {format:?} samples, not 16-bit integer or 32-bit float"),
            ))
        }
    };
    samples.map_err(|error| wav_error(path, error))
}

/// The error of a WAV file that cannot be read.
fn wav_error(path: &Path, error: hound::Error) -> EngineError {
    match error {
        hound::Error::IoError(error) => EngineError::from_io(path, &error),
        other => unsupported(path, &other.to_string()),
    }
}

/// `UNSUPPORTED_FORMAT` for a kit file whose content does not follow 05 §8.
fn unsupported(path: &Path, detail: &str) -> EngineError {
    EngineError::UnsupportedFormat(format!("{}: {detail}", path.display()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use hound::{WavSpec, WavWriter};
    use serde_json::{json, Value};
    use std::path::PathBuf;
    use tempfile::TempDir;

    /// The kits of the repository.
    fn repository_kits_dir() -> PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../assets/kits")
    }

    /// The file of each row (05 §8.3).
    const FILES: [&str; KIT_ROWS] = [
        "low.wav",
        "snare.wav",
        "clap.wav",
        "hat-closed.wav",
        "hat-open.wav",
        "tom.wav",
        "accent.wav",
        "extra.wav",
    ];

    fn spec(channels: u16, sample_rate: u32, bits: u16, format: SampleFormat) -> WavSpec {
        WavSpec {
            channels,
            sample_rate,
            bits_per_sample: bits,
            sample_format: format,
        }
    }

    /// Writes a mono 32-bit float WAV at 48 000 Hz.
    fn write_float_wav(path: &Path, samples: &[f32]) {
        let mut writer = WavWriter::create(path, spec(1, SAMPLE_RATE, 32, SampleFormat::Float))
            .expect("the test can create a WAV");
        for sample in samples {
            writer.write_sample(*sample).expect("the test can write");
        }
        writer.finalize().expect("the test can finish the WAV");
    }

    /// Writes a WAV with `frames` frames of silence and the given format.
    fn write_wav_with(path: &Path, spec: WavSpec, frames: usize) {
        let mut writer = WavWriter::create(path, spec).expect("the test can create a WAV");
        for _ in 0..frames * usize::from(spec.channels) {
            match (spec.sample_format, spec.bits_per_sample) {
                (SampleFormat::Float, _) => writer.write_sample(0.0_f32),
                (SampleFormat::Int, 16) => writer.write_sample(0_i16),
                (SampleFormat::Int, _) => writer.write_sample(0_i32),
            }
            .expect("the test can write");
        }
        writer.finalize().expect("the test can finish the WAV");
    }

    /// Writes the `kit.json` of a kit with these choke groups.
    fn write_kit_json(dir: &Path, groups: [Option<&str>; KIT_ROWS]) {
        let sounds: Vec<Value> = (0..KIT_ROWS)
            .map(|row| {
                json!({
                    "role": KIT_ROLES[row],
                    "file": FILES[row],
                    "name": { "es": "Sonido", "en": "Sound" },
                    "chokeGroup": groups[row],
                })
            })
            .collect();
        let kit = json!({ "id": "test", "name": { "es": "Kit", "en": "Kit" }, "sounds": sounds });
        fs::write(dir.join("kit.json"), kit.to_string()).expect("the test can write kit.json");
    }

    /// A kits folder with the three kits: each sound is one frame of value 1.0.
    fn test_kits() -> TempDir {
        let root = TempDir::new().expect("the test can create a folder");
        for id in KIT_IDS {
            let dir = root.path().join(id);
            fs::create_dir(&dir).expect("the test can create a folder");
            write_kit_json(&dir, [None; KIT_ROWS]);
            for file in FILES {
                write_float_wav(&dir.join(file), &[1.0]);
            }
        }
        root
    }

    fn assert_fails_with(root: &TempDir, code: &str) {
        let result = load_kits(root.path());
        assert_eq!(
            result.as_ref().map(|_| ()).map_err(EngineError::code),
            Err(code),
            "{result:?}"
        );
    }

    // ---- The kits of the repository (08 §7, contract «Kits») ----

    #[test]
    fn kits_contract_the_three_kits_load_with_the_choke_groups_of_05_8() {
        let kits = load_kits(&repository_kits_dir()).expect("the kits of the repository load");
        let groups = |kit: &Kit| kit.sounds.each_ref().map(|sound| sound.choke_group);
        let hat = [0, 0, 0, 1, 1, 0, 0, 0];
        assert_eq!(groups(&kits.kits[0]), hat, "acoustic");
        assert_eq!(groups(&kits.kits[1]), hat, "electronic");
        assert_eq!(groups(&kits.kits[2]), [0; KIT_ROWS], "percussion");

        for kit in &kits.kits {
            for sound in &kit.sounds {
                assert!(!sound.samples.is_empty());
                assert!(sound.samples.len() <= MAX_KIT_SOUND_FRAMES);
                assert!(sound.samples.iter().all(|x| (-1.0..1.0).contains(x)));
            }
        }
        // The acoustic kick lasts 0.40 s (05 §8.4) and peaks at 0.8.
        let kick = &kits.kits[0].sounds[0].samples;
        assert_eq!(kick.len(), 19_200);
        let peak = kick.iter().fold(0.0_f32, |max, x| max.max(x.abs()));
        assert!((peak - 0.8).abs() < 1e-4, "{peak}");
    }

    #[test]
    fn the_kit_json_files_are_those_of_05_8_2_and_05_8_3() {
        let sound = |role: &str, file: &str, es: &str, en: &str, group: Option<&str>| json!({ "role": role, "file": file, "name": { "es": es, "en": en }, "chokeGroup": group });
        let expected = [
            json!({
                "id": "acoustic",
                "name": { "es": "Batería acústica", "en": "Acoustic drums" },
                "sounds": [
                    sound("low", "low.wav", "Bombo", "Kick", None),
                    sound("snare", "snare.wav", "Caja", "Snare", None),
                    sound("clap", "clap.wav", "Aro", "Rimshot", None),
                    sound("hatClosed", "hat-closed.wav", "Charles cerrado", "Closed hi-hat", Some("hat")),
                    sound("hatOpen", "hat-open.wav", "Charles abierto", "Open hi-hat", Some("hat")),
                    sound("tom", "tom.wav", "Tom", "Tom", None),
                    sound("accent", "accent.wav", "Crash", "Crash", None),
                    sound("extra", "extra.wav", "Ride", "Ride", None),
                ]
            }),
            json!({
                "id": "electronic",
                "name": { "es": "Batería electrónica", "en": "Electronic drums" },
                "sounds": [
                    sound("low", "low.wav", "Bombo", "Kick", None),
                    sound("snare", "snare.wav", "Caja", "Snare", None),
                    sound("clap", "clap.wav", "Palmada", "Clap", None),
                    sound("hatClosed", "hat-closed.wav", "Charles cerrado", "Closed hi-hat", Some("hat")),
                    sound("hatOpen", "hat-open.wav", "Charles abierto", "Open hi-hat", Some("hat")),
                    sound("tom", "tom.wav", "Tom", "Tom", None),
                    sound("accent", "accent.wav", "Platillo", "Cymbal", None),
                    sound("extra", "extra.wav", "Cencerro", "Cowbell", None),
                ]
            }),
            json!({
                "id": "percussion",
                "name": { "es": "Percusión", "en": "Percussion" },
                "sounds": [
                    sound("low", "low.wav", "Cajón grave", "Cajón low", None),
                    sound("snare", "snare.wav", "Cajón agudo", "Cajón high", None),
                    sound("clap", "clap.wav", "Palmas", "Handclaps", None),
                    sound("hatClosed", "hat-closed.wav", "Shaker", "Shaker", None),
                    sound("hatOpen", "hat-open.wav", "Pandereta", "Tambourine", None),
                    sound("tom", "tom.wav", "Bongó", "Bongo", None),
                    sound("accent", "accent.wav", "Campana", "Bell", None),
                    sound("extra", "extra.wav", "Claves", "Claves", None),
                ]
            }),
        ];
        for (id, expected) in KIT_IDS.iter().zip(expected) {
            let path = repository_kits_dir().join(id).join("kit.json");
            let text = fs::read_to_string(&path).expect("kit.json exists");
            let actual: Value = serde_json::from_str(&text).expect("kit.json is JSON");
            assert_eq!(actual, expected, "{id}");
        }
    }

    // ---- Reading and choke groups ----

    #[test]
    fn kit_index_follows_the_order_of_the_kits() {
        assert_eq!(kit_index("acoustic"), Some(0));
        assert_eq!(kit_index("electronic"), Some(1));
        assert_eq!(kit_index("percussion"), Some(2));
        assert_eq!(kit_index("jazz"), None);
    }

    #[test]
    fn reads_float_sounds_as_they_are() {
        let root = test_kits();
        write_float_wav(&root.path().join("electronic/tom.wav"), &[1.0, -0.25, 0.5]);
        let kits = load_kits(root.path()).expect("the test kits load");
        assert_eq!(kits.kits[0].sounds[0].samples, vec![1.0]);
        assert_eq!(kits.kits[1].sounds[5].samples, vec![1.0, -0.25, 0.5]);
    }

    #[test]
    fn reads_16_bit_sounds_divided_by_32768() {
        let root = test_kits();
        let path = root.path().join("percussion/extra.wav");
        let mut writer = WavWriter::create(&path, spec(1, SAMPLE_RATE, 16, SampleFormat::Int))
            .expect("the test can create a WAV");
        for value in [16_384_i16, -32_768, 32_767, 0] {
            writer.write_sample(value).expect("the test can write");
        }
        writer.finalize().expect("the test can finish the WAV");

        let kits = load_kits(root.path()).expect("the test kits load");
        assert_eq!(
            kits.kits[2].sounds[7].samples,
            vec![0.5, -1.0, 32_767.0 / 32_768.0, 0.0]
        );
    }

    #[test]
    fn numbers_each_choke_group_from_1_in_row_order() {
        let root = test_kits();
        let groups = [
            Some("b"),
            None,
            Some("a"),
            Some("b"),
            None,
            Some("a"),
            Some("c"),
            None,
        ];
        write_kit_json(&root.path().join("percussion"), groups);
        let kits = load_kits(root.path()).expect("the test kits load");
        let numbers = kits.kits[2]
            .sounds
            .each_ref()
            .map(|sound| sound.choke_group);
        assert_eq!(numbers, [1, 0, 2, 1, 0, 2, 3, 0]);
        // Each kit numbers its own groups.
        assert_eq!(kits.kits[0].sounds[0].choke_group, 0);
    }

    #[test]
    fn accepts_a_sound_of_exactly_4_seconds() {
        let root = test_kits();
        let path = root.path().join("acoustic/accent.wav");
        write_wav_with(
            &path,
            spec(1, SAMPLE_RATE, 16, SampleFormat::Int),
            MAX_KIT_SOUND_FRAMES,
        );
        let kits = load_kits(root.path()).expect("a 4-second sound loads");
        assert_eq!(kits.kits[0].sounds[6].samples.len(), 192_000);
    }

    // ---- Incomplete kits and sounds that do not follow 05 §8.3 ----

    #[test]
    fn fails_when_a_kit_is_missing() {
        let root = test_kits();
        fs::remove_dir_all(root.path().join("percussion")).expect("the test can delete");
        assert_fails_with(&root, "NOT_FOUND");
    }

    #[test]
    fn fails_when_a_sound_file_is_missing() {
        let root = test_kits();
        fs::remove_file(root.path().join("electronic/hat-open.wav")).expect("the test can delete");
        assert_fails_with(&root, "NOT_FOUND");
    }

    #[test]
    fn fails_when_kit_json_has_fewer_than_8_sounds() {
        let root = test_kits();
        let path = root.path().join("acoustic/kit.json");
        let mut kit: Value =
            serde_json::from_str(&fs::read_to_string(&path).expect("kit.json exists"))
                .expect("kit.json is JSON");
        kit["sounds"]
            .as_array_mut()
            .expect("sounds is a list")
            .pop();
        fs::write(&path, kit.to_string()).expect("the test can write");
        assert_fails_with(&root, "UNSUPPORTED_FORMAT");
    }

    #[test]
    fn fails_when_the_roles_are_not_in_the_order_of_05_8_1() {
        let root = test_kits();
        let path = root.path().join("acoustic/kit.json");
        let text = fs::read_to_string(&path).expect("kit.json exists");
        let swapped = text
            .replace("\"hatClosed\"", "\"swap\"")
            .replace("\"hatOpen\"", "\"hatClosed\"")
            .replace("\"swap\"", "\"hatOpen\"");
        fs::write(&path, swapped).expect("the test can write");
        assert_fails_with(&root, "UNSUPPORTED_FORMAT");
    }

    #[test]
    fn fails_when_kit_json_is_not_a_kit() {
        let root = test_kits();
        fs::write(root.path().join("electronic/kit.json"), "{ \"id\": ")
            .expect("the test can write");
        assert_fails_with(&root, "UNSUPPORTED_FORMAT");
    }

    #[test]
    fn fails_with_a_stereo_sound() {
        let root = test_kits();
        let path = root.path().join("acoustic/snare.wav");
        write_wav_with(&path, spec(2, SAMPLE_RATE, 16, SampleFormat::Int), 10);
        assert_fails_with(&root, "UNSUPPORTED_FORMAT");
    }

    #[test]
    fn fails_with_a_sound_at_44100_hz() {
        let root = test_kits();
        let path = root.path().join("acoustic/snare.wav");
        write_wav_with(&path, spec(1, 44_100, 16, SampleFormat::Int), 10);
        assert_fails_with(&root, "UNSUPPORTED_FORMAT");
    }

    #[test]
    fn fails_with_a_24_bit_sound() {
        let root = test_kits();
        let path = root.path().join("acoustic/snare.wav");
        write_wav_with(&path, spec(1, SAMPLE_RATE, 24, SampleFormat::Int), 10);
        assert_fails_with(&root, "UNSUPPORTED_FORMAT");
    }

    #[test]
    fn fails_with_a_sound_longer_than_4_seconds() {
        let root = test_kits();
        let path = root.path().join("acoustic/accent.wav");
        write_wav_with(
            &path,
            spec(1, SAMPLE_RATE, 16, SampleFormat::Int),
            MAX_KIT_SOUND_FRAMES + 1,
        );
        assert_fails_with(&root, "UNSUPPORTED_FORMAT");
    }

    #[test]
    fn fails_with_a_file_that_is_not_a_wav() {
        let root = test_kits();
        fs::write(root.path().join("percussion/low.wav"), "not a wav").expect("the test can write");
        assert_fails_with(&root, "UNSUPPORTED_FORMAT");
    }
}
