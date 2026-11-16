# Carabo · Especificación · 05 — Datos y formato de proyecto

> **PROMPT PARA EL AGENTE — léelo antes de seguir**
>
> Eres un agente de programación que trabaja en el repositorio de **Carabo**, un DAW de escritorio para Windows 11. Este documento define **todas las estructuras de datos y todos los archivos** del programa: el modelo del proyecto, el archivo de proyecto, la carpeta de proyecto, los ajustes, los kits, los ritmos hechos y la instantánea que la interfaz envía al motor.
>
> - Léelo antes de leer o escribir cualquier estructura de datos o archivo.
> - Los nombres de campos, los tipos, los rangos y los valores iniciales son **exactos**: cópialos tal cual. No añadas campos ni cambies nombres.
> - Este documento manda sobre los demás en cuanto a datos (00 §5). Qué operaciones cambian los datos está en 02; quién los maneja, en 04; cómo los usa el motor, en 06.
> - Los nombres en `MAYÚSCULAS` son constantes de 00 §4. Los términos (clip, bloque, segmento, tapar…) están en el glosario (00 §7).
> - Lo marcado **[1.1]** no se implementa en la versión 1.0, pero la 1.0 no debe impedirlo.

---

## 1. Dónde vive cada dato

| Dato | Dónde | Formato | Quién lo escribe |
|---|---|---|---|
| Estado del proyecto | Memoria de la interfaz; se guarda en `project.carabo` | JSON (§5) | La interfaz serializa; el programa nativo escribe el archivo |
| Audio grabado o importado | `audio/*.wav` en la carpeta de proyecto | WAV, 48 000 Hz, coma flotante de 32 bits, 1 o 2 canales | El motor |
| Datos para dibujar ondas | `peaks/*.peaks` en la carpeta de proyecto | Binario (§6.3) | El motor |
| Copia de recuperación | `recovery/project.recovery.carabo` | Igual que `project.carabo` | La interfaz serializa; el programa nativo escribe |
| Ajustes del programa | `settings.json` en la carpeta de configuración (§7.1) | JSON | La interfaz serializa; el programa nativo escribe |
| Sesión | `session.json` en la carpeta de configuración (§7.2) | JSON | El programa nativo |
| Registro | `logs/carabo.log` en la carpeta de datos locales (§7.3) | Texto | El programa nativo |
| Kits | `assets/kits/<kit>/` en los recursos del programa (§8) | JSON y WAV | Vienen con el programa; solo lectura |
| Ritmos hechos | `assets/presets/presets.json` (§9) | JSON | Vienen con el programa; solo lectura |
| Instantánea del motor | Memoria; viaja por el puente | JSON (§10) | La interfaz |

«La interfaz» es el código TypeScript; «el programa nativo» es el código Rust de `src-tauri`; «el motor» es la biblioteca Rust `carabo-engine` (04).

## 2. Unidades y conversiones

Todas las conversiones del programa usan estas fórmulas y solo estas. Se implementan una vez en TypeScript y una vez en Rust (04), con pruebas que comprueban que dan los mismos resultados (08).

### 2.1 Unidades

| Unidad | Qué mide | Tipo |
|---|---|---|
| Frame | Audio. 48 000 frames = 1 segundo | Entero ≥ 0 |
| Tick | Tiempo musical. `PPQ` = 960 ticks = 1 negra | Entero ≥ 0 |
| Paso | Una semicorchea = `TICKS_PER_STEP` = 240 ticks | Entero ≥ 0 |
| Compás | Depende de la indicación de compás (tabla siguiente) | Entero ≥ 0; el compás 1 de la pantalla es el 0 |

| Indicación | Ticks por compás | Pasos por compás | Pulsos por compás | Ticks por pulso |
|---|---|---|---|---|
| `4/4` | 3 840 | 16 | 4 | 960 |
| `3/4` | 2 880 | 12 | 3 | 960 |
| `6/8` | 2 880 | 12 | 6 | 480 |

### 2.2 Fórmulas

El tempo (`tempoBpm`) son negras por minuto, siempre, sea cual sea el compás.

```text
framesPerTick      = 3000 / tempoBpm                        (número real: 48000 · 60 / 960 / tempoBpm)
tickToFrame(t)     = round(t · 3000 / tempoBpm)
frameToTick(f)     = round(f · tempoBpm / 3000)
stepToFrame(s)     = round(s · 720000 / tempoBpm)           (s = índice de paso desde el principio de la canción)
barToTick(b)       = b · ticksPerBar
barToStep(b)       = b · stepsPerBar
maxSongTick        = floor(MAX_SONG_FRAMES · tempoBpm / 3000)

Presentación de una posición en ticks (RF-113):
  compás  = floor(tick / ticksPerBar) + 1
  pulso   = floor((tick mod ticksPerBar) / ticksPerPulse) + 1
  tiempo  = tickToFrame(tick) / 48000 segundos
```

- `round` redondea al entero más cercano; los valores a medias (`x,5`) suben. Todos los operandos son no negativos.
- Las conversiones se calculan siempre **desde el principio de la canción**, nunca acumulando incrementos, para no arrastrar errores de redondeo.
- Ejemplo: a 120 negras por minuto, un paso son 6 000 frames; a 97, `stepToFrame(1) = 7423` y `stepToFrame(2) = 14845`.

### 2.3 Dónde termina un elemento

```text
Clip:    startFrame = tickToFrame(clip.startTick)
         endFrame   = startFrame + clip.lengthFrames
Bloque:  startFrame = stepToFrame(barToStep(block.startBar))
         endFrame   = stepToFrame(barToStep(block.startBar + block.lengthBars))
```

Ningún elemento puede tener `endFrame > MAX_SONG_FRAMES` (RF-112).

## 3. Modelo de datos del proyecto

El modelo se define en TypeScript en un único archivo (ruta en 07). El archivo de proyecto es la serialización directa de este modelo. El proyecto tiene dos partes:

- **El documento** (`ProjectDoc`): todo lo que entra en el historial de deshacer.
- **El estado de vista** (`ViewState`): se guarda con el proyecto pero no entra en el historial ni marca cambios sin guardar (RF-031).

### 3.1 Tipos

```ts
// ---- Tipos básicos ----
type TimeSignature = "4/4" | "3/4" | "6/8";
type KitId = "acoustic" | "electronic" | "percussion";
type Zoom = 6 | 12 | 24 | 48 | 96 | 192;

// ---- Proyecto ----
interface ProjectFile extends ProjectDoc {
  view: ViewState;
}

interface ProjectDoc {
  formatVersion: 1;
  tempoBpm: number;              // entero, TEMPO_MIN..TEMPO_MAX
  timeSignature: TimeSignature;
  tracks: Track[];               // 0..MAX_TRACKS, en el orden de las pistas
  sources: AudioSource[];        // exactamente los archivos de audio que algún clip usa
  master: MasterStrip;
  nextId: number;                // entero ≥ 1: siguiente número para un identificador
  nextZ: number;                 // entero ≥ 1: siguiente valor de z
}

interface ViewState {
  metronomeEnabled: boolean;
  loopEnabled: boolean;
  loopStartBar: number;          // entero ≥ 0
  loopEndBar: number;            // entero > loopStartBar (el tramo es [loopStartBar, loopEndBar))
  snapEnabled: boolean;
  zoom: Zoom;                    // píxeles por negra
}

// ---- Pistas ----
type Track = RhythmTrack | AudioTrack;

interface TrackBase {
  id: string;                    // "trk-<n>"
  type: "rhythm" | "audio";
  name: string;                  // 1..NAME_MAX_CHARS caracteres
  colorIndex: number;            // entero 0..7: color de la paleta de pistas (03)
  strip: ChannelStrip;
}

interface RhythmTrack extends TrackBase {
  type: "rhythm";
  kitId: KitId;
  rows: RowSettings[];           // exactamente KIT_ROWS (8), en el orden de filas del kit
  patterns: Pattern[];           // 1..MAX_PATTERNS_PER_TRACK, en orden de creación
  blocks: PatternBlock[];        // en cualquier orden
}

interface AudioTrack extends TrackBase {
  type: "audio";
  clips: AudioClip[];            // en cualquier orden
}

// ---- Ritmo ----
interface RowSettings {
  gainDb: number;                // entero, ROW_GAIN_MIN_DB..ROW_GAIN_MAX_DB
  mute: boolean;
}

interface Pattern {
  id: string;                    // "pat-<n>"
  name: string;                  // 1..NAME_MAX_CHARS caracteres
  bars: 1 | 2 | 4;
  steps: number[][];             // 8 filas × (bars · stepsPerBar) pasos; cada valor es un nivel 0..3
}

interface PatternBlock {
  id: string;                    // "blk-<n>"
  patternId: string;             // un patrón de la misma pista
  startBar: number;              // entero ≥ 0
  lengthBars: number;            // entero ≥ 1
  z: number;                     // entero ≥ 1; mayor = más encima
}

// ---- Audio ----
interface AudioClip {
  id: string;                    // "clip-<n>"
  sourceId: string;              // un elemento de ProjectDoc.sources
  startTick: number;             // entero ≥ 0
  lengthFrames: number;          // entero ≥ MIN_CLIP_FRAMES: lo que el clip ocupa en la canción
  regionStartFrame: number;      // entero ≥ 0: primer frame del archivo que suena
  regionLengthFrames: number;    // entero ≥ MIN_CLIP_FRAMES: longitud de la región
  repeat: boolean;               // true: la región se repite hasta completar lengthFrames
  z: number;                     // entero ≥ 1; mayor = más encima
}

interface AudioSource {
  id: string;                    // "src-<n>"
  kind: "take" | "import";
  file: string;                  // ruta relativa a la carpeta de proyecto, con "/": "audio/take-0001.wav"
  name: string;                  // nombre visible: "Toma 1" o el nombre del archivo importado sin extensión
  frames: number;                // entero ≥ 1: longitud del archivo en frames
  channels: 1 | 2;
}

// ---- Mezcla ----
interface ChannelStrip {
  gainDb: number;                // FADER_MIN_DB..FADER_MAX_DB en pasos de 0,5; -60 significa ganancia 0 («−∞»)
  pan: number;                   // entero -100..100
  mute: boolean;
  solo: boolean;
  effects: EffectUnit[];         // en la 1.0, exactamente [EqUnit, ReverbUnit], en ese orden
}

type EffectUnit = EqUnit | ReverbUnit;

interface EqUnit {
  type: "eq";
  enabled: boolean;
  lowDb: number;                 // EQ_GAIN_MIN_DB..EQ_GAIN_MAX_DB en pasos de 0,5
  midDb: number;
  highDb: number;
}

interface ReverbUnit {
  type: "reverb";
  enabled: boolean;
  amount: number;                // entero 0..100
  size: number;                  // entero 0..100
}

interface MasterStrip {
  gainDb: number;                // como ChannelStrip.gainDb
}
```

### 3.2 Identificadores y contadores

- Todo identificador tiene la forma `<prefijo>-<n>`, con los prefijos `trk`, `pat`, `blk`, `clip` y `src`. `<n>` es el valor de `nextId` en el momento de crearlo; después `nextId` aumenta en 1. Un mismo contador sirve para todos los prefijos, así que `<n>` no se repite dentro de un proyecto.
- Cada vez que un bloque o un clip se crea, se mueve o se redimensiona, recibe `z = nextZ` y `nextZ` aumenta en 1 (RF-309).
- `nextId` y `nextZ` forman parte del documento: al deshacer, vuelven a su valor anterior.
- Los identificadores no significan nada más: no se muestran al usuario ni se usan para ordenar.

### 3.3 Cómo suena un clip

Un clip no contiene audio. Dice qué frames de su archivo suenan y en qué momento:

```text
Para t = 0 .. lengthFrames - 1   (t = frame contado desde el principio del clip):
   frameDelArchivo(t) = regionStartFrame + (t mod regionLengthFrames)
```

- Con `repeat = false`, `lengthFrames` es igual a `regionLengthFrames` y el `mod` no tiene efecto: suena la región una vez.
- Con `repeat = true`, la región suena repetida; la última repetición queda cortada si `lengthFrames` no es múltiplo de `regionLengthFrames`.

Operaciones de edición sobre estos campos (los requisitos están en 02 §6):

| Operación | Efecto sobre el clip |
|---|---|
| Mover a la posición `T` (ticks) | `startTick = T` |
| Llevar el borde izquierdo al tick `T`, con `repeat = false` | `d = tickToFrame(T) − tickToFrame(startTick)` (positivo si el borde va a la derecha); `startTick = T`; `regionStartFrame += d`; `regionLengthFrames −= d`; `lengthFrames −= d` |
| Llevar el borde derecho al tick `T`, con `repeat = false` | `L = tickToFrame(T) − tickToFrame(startTick)`; `regionLengthFrames = L`; `lengthFrames = L` |
| Llevar el borde derecho al tick `T`, con `repeat = true` | `L = tickToFrame(T) − tickToFrame(startTick)`; `lengthFrames = L` |
| Activar «Repetir» | `repeat = true` |
| Desactivar «Repetir» | `repeat = false`; `lengthFrames = regionLengthFrames` |
| Partir en el tick `C` del cursor (`repeat = false`) | `s = tickToFrame(C) − tickToFrame(startTick)`. Clip izquierdo: `regionLengthFrames = lengthFrames = s`. Clip derecho nuevo: mismo `sourceId`; `startTick = C`; `regionStartFrame = original + s`; `regionLengthFrames = lengthFrames = original − s` |
| Duplicar | Copia con `startTick = frameToTick(endFrame)` y el resto de campos iguales |

Límites de estas operaciones:

- Borde izquierdo: `regionStartFrame ≥ 0` (no hay audio antes del principio del archivo) y `regionLengthFrames ≥ MIN_CLIP_FRAMES`.
- Borde derecho sin «Repetir»: `regionStartFrame + L ≤ source.frames` y `L ≥ MIN_CLIP_FRAMES`.
- Borde derecho con «Repetir»: `L ≥ MIN_CLIP_FRAMES`.
- Siempre: `startTick ≥ 0` y `endFrame ≤ MAX_SONG_FRAMES`.
- Si el valor pedido queda fuera de un límite, se usa el valor permitido más cercano.

Como las posiciones se calculan siempre a partir de ticks, devolver un borde a su tick original restaura exactamente los valores originales. En todas las operaciones el elemento afectado (y el clip nuevo, si lo hay) recibe un `z` nuevo.

### 3.4 Cómo suena un bloque

Un bloque hace sonar su patrón repetido desde el primer paso del patrón:

```text
blockStartStep = barToStep(startBar)
blockLenSteps  = lengthBars · stepsPerBar
patternLen     = pattern.bars · stepsPerBar
Para s = 0 .. blockLenSteps - 1:
   en el paso de canción (blockStartStep + s) suena el paso (s mod patternLen) del patrón
```

En cada paso, cada fila con nivel distinto de 0 dispara su sonido con la ganancia `LEVEL_GAINS[nivel]` multiplicada por la ganancia de la fila, salvo que la fila esté silenciada (06).

### 3.5 Valores iniciales

| Elemento | Valores iniciales |
|---|---|
| Proyecto nuevo | `formatVersion: 1`, `tempoBpm: 120`, `timeSignature: "4/4"`, `tracks: []`, `sources: []`, `master: { gainDb: 0 }`, `nextId: 1`, `nextZ: 1` |
| Estado de vista | `metronomeEnabled: false`, `loopEnabled: false`, `loopStartBar: 0`, `loopEndBar: 4`, `snapEnabled: true`, `zoom: 24` |
| `ChannelStrip` | `gainDb: 0`, `pan: 0`, `mute: false`, `solo: false`, `effects: [ {type:"eq", enabled:false, lowDb:0, midDb:0, highDb:0}, {type:"reverb", enabled:false, amount:20, size:50} ]` |
| `RowSettings` | `gainDb: 0`, `mute: false` |
| Pista de ritmo nueva | `kitId: "acoustic"`; 8 `RowSettings`; un patrón; un bloque (RF-201); `colorIndex` = número de pistas que había antes de crearla, módulo 8 |
| Pista de audio nueva | `clips: []`; `colorIndex` como arriba |
| Patrón nuevo | `bars: 1`; `steps`: 8 filas de `stepsPerBar` ceros |
| Clip nuevo (grabado o importado) | `repeat: false`; `regionLengthFrames = lengthFrames` |

### 3.6 Campos de la versión 1.1

La 1.1 añade estos campos y sube `formatVersion` a 2. Al abrir un archivo de versión 1, la 1.1 los añade con su valor inicial. La 1.0 rechaza los archivos de versión 2 (RF-034).

| Dónde | Campo | Tipo y rango | Valor inicial | Requisito |
|---|---|---|---|---|
| `Pattern` | `swing` | entero 0..100 | 0 | RF-251 |
| `RowSettings` | `customSourceId` | `string \| null` | `null` | RF-252 |
| `AudioSource.kind` | valor `"sample"` | — | — | RF-252 |
| `AudioClip` | `gainDb` | −24..24 en pasos de 0,5 | 0 | RF-352 |
| `AudioClip` | `fadeInFrames`, `fadeOutFrames` | entero 0..`lengthFrames / 2` | 0 | RF-350 |
| `ProjectDoc` | `markers` | `{ id: "mrk-<n>", name: string, bar: number }[]` | `[]` | RF-353 |
| `ChannelStrip` | `effects` pasa a ser `[eq, compressor, delay, reverb]` | `CompressorUnit { type:"compressor", enabled, thresholdDb, ratio, makeupDb }` y `DelayUnit { type:"delay", enabled, time: "1/4"\|"1/8d"\|"1/8"\|"1/16", feedback, amount }` | Los de RF-450 y RF-451 | RF-450, RF-451 |
| `MasterStrip` | `limiterEnabled` | booleano | `false` | RF-452 |
| `ViewState` | `punchEnabled` | booleano | `false` | RF-115 |

## 4. Invariantes del proyecto

Un proyecto es válido solo si cumple el esquema de §5.2 **y** todas estas reglas. Se comprueban al abrir un archivo (si falla alguna, el archivo se trata como dañado, RF-021) y, en las pruebas, después de cada operación de edición (08).

| Id | Regla |
|---|---|
| INV-01 | `tracks.length ≤ MAX_TRACKS`. |
| INV-02 | Todos los `id` del proyecto son distintos y su número es menor que `nextId`. |
| INV-03 | En una pista de ritmo: `rows.length = 8`; `1 ≤ patterns.length ≤ MAX_PATTERNS_PER_TRACK`. |
| INV-04 | En cada patrón: `steps` tiene 8 filas y cada fila tiene exactamente `bars · stepsPerBar` valores, todos enteros de 0 a 3. |
| INV-05 | Cada `block.patternId` es el `id` de un patrón de la misma pista. |
| INV-06 | Cada `clip.sourceId` es el `id` de un elemento de `sources`. |
| INV-07 | En cada clip: `regionStartFrame + regionLengthFrames ≤ source.frames`. |
| INV-08 | En cada clip: si `repeat` es `false`, `lengthFrames = regionLengthFrames`. |
| INV-09 | Para cada bloque y cada clip, `endFrame ≤ MAX_SONG_FRAMES` (§2.3). |
| INV-10 | Dentro de un carril, todos los `z` son distintos; todos los `z` del proyecto son menores que `nextZ`. |
| INV-11 | `sources` no contiene elementos sin usar: cada uno está referido por al menos un clip. No hay dos con el mismo `file`. |
| INV-12 | En cada `ChannelStrip`, `effects` tiene exactamente dos elementos: el primero de tipo `"eq"` y el segundo de tipo `"reverb"`. |
| INV-13 | El nombre de cada pista y de cada patrón tiene entre 1 y `NAME_MAX_CHARS` caracteres y no empieza ni termina con espacio. El nombre de un archivo de audio (`AudioSource.name`) tiene entre 1 y 255 caracteres. |
| INV-14 | `view.loopEndBar > view.loopStartBar ≥ 0`. |

**Cómo se mantiene INV-11.** Toda operación que elimina clips (eliminar un clip, eliminar una pista, deshacer una grabación) quita de `sources`, dentro del mismo paso de historial, los elementos que quedan sin usar. Los archivos no se borran en ese momento (RF-029).

## 5. Archivo de proyecto `project.carabo`

### 5.1 Reglas de escritura

- Es la serialización JSON de un `ProjectFile` (§3.1).
- Codificación UTF-8 sin marca de orden de bytes, saltos de línea `\n`, sangría de 2 espacios.
- Las claves se escriben en el orden en que aparecen en las interfaces de §3.1, para que dos guardados seguidos del mismo estado produzcan archivos idénticos.
- Cada fila de `steps` se escribe en una sola línea.
- Se escribe de forma atómica (RNF-13): primero `project.carabo.tmp` en la misma carpeta y después se sustituye el definitivo.

### 5.2 Esquema

El esquema se guarda en el repositorio como archivo (ruta en 07) y es el que usa la validación al abrir.

```json
{
  "$schema": "https://json-schema.org/draft/2020-12/schema",
  "$id": "urn:carabo:schema:project:1",
  "title": "Archivo de proyecto de Carabo, formatVersion 1",
  "type": "object",
  "additionalProperties": false,
  "required": ["formatVersion", "tempoBpm", "timeSignature", "tracks", "sources", "master", "nextId", "nextZ", "view"],
  "properties": {
    "formatVersion": { "const": 1 },
    "tempoBpm": { "type": "integer", "minimum": 40, "maximum": 240 },
    "timeSignature": { "enum": ["4/4", "3/4", "6/8"] },
    "tracks": {
      "type": "array",
      "maxItems": 16,
      "items": { "oneOf": [{ "$ref": "#/$defs/rhythmTrack" }, { "$ref": "#/$defs/audioTrack" }] }
    },
    "sources": { "type": "array", "items": { "$ref": "#/$defs/source" } },
    "master": {
      "type": "object",
      "additionalProperties": false,
      "required": ["gainDb"],
      "properties": { "gainDb": { "$ref": "#/$defs/faderDb" } }
    },
    "nextId": { "type": "integer", "minimum": 1 },
    "nextZ": { "type": "integer", "minimum": 1 },
    "view": { "$ref": "#/$defs/view" }
  },
  "$defs": {
    "name": { "type": "string", "minLength": 1, "maxLength": 40 },
    "faderDb": { "type": "number", "minimum": -60, "maximum": 6, "multipleOf": 0.5 },
    "eqDb": { "type": "number", "minimum": -12, "maximum": 12, "multipleOf": 0.5 },
    "percent": { "type": "integer", "minimum": 0, "maximum": 100 },
    "z": { "type": "integer", "minimum": 1 },
    "strip": {
      "type": "object",
      "additionalProperties": false,
      "required": ["gainDb", "pan", "mute", "solo", "effects"],
      "properties": {
        "gainDb": { "$ref": "#/$defs/faderDb" },
        "pan": { "type": "integer", "minimum": -100, "maximum": 100 },
        "mute": { "type": "boolean" },
        "solo": { "type": "boolean" },
        "effects": {
          "type": "array",
          "prefixItems": [{ "$ref": "#/$defs/eq" }, { "$ref": "#/$defs/reverb" }],
          "items": false,
          "minItems": 2,
          "maxItems": 2
        }
      }
    },
    "eq": {
      "type": "object",
      "additionalProperties": false,
      "required": ["type", "enabled", "lowDb", "midDb", "highDb"],
      "properties": {
        "type": { "const": "eq" },
        "enabled": { "type": "boolean" },
        "lowDb": { "$ref": "#/$defs/eqDb" },
        "midDb": { "$ref": "#/$defs/eqDb" },
        "highDb": { "$ref": "#/$defs/eqDb" }
      }
    },
    "reverb": {
      "type": "object",
      "additionalProperties": false,
      "required": ["type", "enabled", "amount", "size"],
      "properties": {
        "type": { "const": "reverb" },
        "enabled": { "type": "boolean" },
        "amount": { "$ref": "#/$defs/percent" },
        "size": { "$ref": "#/$defs/percent" }
      }
    },
    "row": {
      "type": "object",
      "additionalProperties": false,
      "required": ["gainDb", "mute"],
      "properties": {
        "gainDb": { "type": "integer", "minimum": -48, "maximum": 6 },
        "mute": { "type": "boolean" }
      }
    },
    "pattern": {
      "type": "object",
      "additionalProperties": false,
      "required": ["id", "name", "bars", "steps"],
      "properties": {
        "id": { "type": "string", "pattern": "^pat-[1-9][0-9]*$" },
        "name": { "$ref": "#/$defs/name" },
        "bars": { "enum": [1, 2, 4] },
        "steps": {
          "type": "array",
          "minItems": 8,
          "maxItems": 8,
          "items": {
            "type": "array",
            "minItems": 12,
            "maxItems": 64,
            "items": { "type": "integer", "minimum": 0, "maximum": 3 }
          }
        }
      }
    },
    "block": {
      "type": "object",
      "additionalProperties": false,
      "required": ["id", "patternId", "startBar", "lengthBars", "z"],
      "properties": {
        "id": { "type": "string", "pattern": "^blk-[1-9][0-9]*$" },
        "patternId": { "type": "string", "pattern": "^pat-[1-9][0-9]*$" },
        "startBar": { "type": "integer", "minimum": 0 },
        "lengthBars": { "type": "integer", "minimum": 1 },
        "z": { "$ref": "#/$defs/z" }
      }
    },
    "rhythmTrack": {
      "type": "object",
      "additionalProperties": false,
      "required": ["id", "type", "name", "colorIndex", "strip", "kitId", "rows", "patterns", "blocks"],
      "properties": {
        "id": { "type": "string", "pattern": "^trk-[1-9][0-9]*$" },
        "type": { "const": "rhythm" },
        "name": { "$ref": "#/$defs/name" },
        "colorIndex": { "type": "integer", "minimum": 0, "maximum": 7 },
        "strip": { "$ref": "#/$defs/strip" },
        "kitId": { "enum": ["acoustic", "electronic", "percussion"] },
        "rows": { "type": "array", "minItems": 8, "maxItems": 8, "items": { "$ref": "#/$defs/row" } },
        "patterns": { "type": "array", "minItems": 1, "maxItems": 32, "items": { "$ref": "#/$defs/pattern" } },
        "blocks": { "type": "array", "items": { "$ref": "#/$defs/block" } }
      }
    },
    "clip": {
      "type": "object",
      "additionalProperties": false,
      "required": ["id", "sourceId", "startTick", "lengthFrames", "regionStartFrame", "regionLengthFrames", "repeat", "z"],
      "properties": {
        "id": { "type": "string", "pattern": "^clip-[1-9][0-9]*$" },
        "sourceId": { "type": "string", "pattern": "^src-[1-9][0-9]*$" },
        "startTick": { "type": "integer", "minimum": 0 },
        "lengthFrames": { "type": "integer", "minimum": 2400 },
        "regionStartFrame": { "type": "integer", "minimum": 0 },
        "regionLengthFrames": { "type": "integer", "minimum": 2400 },
        "repeat": { "type": "boolean" },
        "z": { "$ref": "#/$defs/z" }
      }
    },
    "audioTrack": {
      "type": "object",
      "additionalProperties": false,
      "required": ["id", "type", "name", "colorIndex", "strip", "clips"],
      "properties": {
        "id": { "type": "string", "pattern": "^trk-[1-9][0-9]*$" },
        "type": { "const": "audio" },
        "name": { "$ref": "#/$defs/name" },
        "colorIndex": { "type": "integer", "minimum": 0, "maximum": 7 },
        "strip": { "$ref": "#/$defs/strip" },
        "clips": { "type": "array", "items": { "$ref": "#/$defs/clip" } }
      }
    },
    "source": {
      "type": "object",
      "additionalProperties": false,
      "required": ["id", "kind", "file", "name", "frames", "channels"],
      "properties": {
        "id": { "type": "string", "pattern": "^src-[1-9][0-9]*$" },
        "kind": { "enum": ["take", "import"] },
        "file": { "type": "string", "pattern": "^audio/(take|import)-[0-9]{4}\\.wav$" },
        "name": { "type": "string", "minLength": 1, "maxLength": 255 },
        "frames": { "type": "integer", "minimum": 1 },
        "channels": { "enum": [1, 2] }
      }
    },
    "view": {
      "type": "object",
      "additionalProperties": false,
      "required": ["metronomeEnabled", "loopEnabled", "loopStartBar", "loopEndBar", "snapEnabled", "zoom"],
      "properties": {
        "metronomeEnabled": { "type": "boolean" },
        "loopEnabled": { "type": "boolean" },
        "loopStartBar": { "type": "integer", "minimum": 0 },
        "loopEndBar": { "type": "integer", "minimum": 1 },
        "snapEnabled": { "type": "boolean" },
        "zoom": { "enum": [6, 12, 24, 48, 96, 192] }
      }
    }
  }
}
```

### 5.3 Ejemplo completo

Proyecto «Demo» a 100 negras por minuto en 4/4, con una pista de ritmo (un patrón de rock básico colocado en los compases 1 a 16) y una pista de audio con una toma de 12 segundos grabada desde el compás 5. El equipo tenía un retardo de 50 ms (2 400 frames), que la región del clip descarta.

```json
{
  "formatVersion": 1,
  "tempoBpm": 100,
  "timeSignature": "4/4",
  "tracks": [
    {
      "id": "trk-1",
      "type": "rhythm",
      "name": "Ritmo 1",
      "colorIndex": 0,
      "strip": {
        "gainDb": -3,
        "pan": 0,
        "mute": false,
        "solo": false,
        "effects": [
          { "type": "eq", "enabled": false, "lowDb": 0, "midDb": 0, "highDb": 0 },
          { "type": "reverb", "enabled": false, "amount": 20, "size": 50 }
        ]
      },
      "kitId": "acoustic",
      "rows": [
        { "gainDb": 0, "mute": false },
        { "gainDb": 0, "mute": false },
        { "gainDb": 0, "mute": false },
        { "gainDb": -6, "mute": false },
        { "gainDb": 0, "mute": false },
        { "gainDb": 0, "mute": false },
        { "gainDb": 0, "mute": false },
        { "gainDb": 0, "mute": false }
      ],
      "patterns": [
        {
          "id": "pat-2",
          "name": "Patrón 1",
          "bars": 1,
          "steps": [
            [2, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [2, 0, 2, 0, 2, 0, 2, 0, 2, 0, 2, 0, 2, 0, 2, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
          ]
        }
      ],
      "blocks": [
        { "id": "blk-3", "patternId": "pat-2", "startBar": 0, "lengthBars": 16, "z": 1 }
      ]
    },
    {
      "id": "trk-4",
      "type": "audio",
      "name": "Audio 1",
      "colorIndex": 1,
      "strip": {
        "gainDb": 0,
        "pan": 0,
        "mute": false,
        "solo": false,
        "effects": [
          { "type": "eq", "enabled": false, "lowDb": 0, "midDb": 0, "highDb": 0 },
          { "type": "reverb", "enabled": true, "amount": 25, "size": 60 }
        ]
      },
      "clips": [
        {
          "id": "clip-6",
          "sourceId": "src-5",
          "startTick": 15360,
          "lengthFrames": 573600,
          "regionStartFrame": 2400,
          "regionLengthFrames": 573600,
          "repeat": false,
          "z": 2
        }
      ]
    }
  ],
  "sources": [
    { "id": "src-5", "kind": "take", "file": "audio/take-0001.wav", "name": "Toma 1", "frames": 576000, "channels": 1 }
  ],
  "master": { "gainDb": 0 },
  "nextId": 7,
  "nextZ": 3,
  "view": {
    "metronomeEnabled": false,
    "loopEnabled": false,
    "loopStartBar": 0,
    "loopEndBar": 4,
    "snapEnabled": true,
    "zoom": 24
  }
}
```

Comprobaciones sobre el ejemplo: a 100 negras por minuto, `framesPerTick = 30`; el clip empieza en `tickToFrame(15360) = 460 800` (9,6 segundos, el principio del compás 5) y termina en `1 034 400`. El bloque ocupa los pasos 0 a 255.

## 6. Carpeta de proyecto

### 6.1 Estructura

```text
Demo/                              ← carpeta de proyecto; su nombre es el nombre del proyecto
├── project.carabo                 ← archivo de proyecto (§5)
├── audio/                         ← archivos de audio
│   ├── take-0001.wav              ← toma grabada
│   └── import-0001.wav            ← archivo importado, ya convertido
├── peaks/                         ← datos para dibujar ondas; se pueden regenerar
│   ├── take-0001.peaks
│   └── import-0001.peaks
└── recovery/                      ← copia de recuperación; puede no existir
    └── project.recovery.carabo
```

- El nombre del proyecto no se guarda dentro del archivo: es el nombre de la carpeta.
- Un proyecto que nunca se ha guardado tiene esta misma estructura, sin `project.carabo`, dentro de la carpeta de datos locales: `unsaved/<uuid>/` (§7).
- Mover o copiar la carpeta entera conserva el proyecto. Mover solo `project.carabo` lo deja sin audio (RF-028).

### 6.2 Archivos de audio

| Propiedad | Valor |
|---|---|
| Contenedor | WAV (RIFF) |
| Frecuencia de muestreo | 48 000 Hz |
| Formato de muestra | Coma flotante IEEE de 32 bits (`WAVE_FORMAT_IEEE_FLOAT`) |
| Canales | 1 (tomas y archivos mono) o 2 (archivos estéreo importados) |
| Nombre de una toma | `take-NNNN.wav`, con `NNNN` de cuatro cifras |
| Nombre de un importado | `import-NNNN.wav` |

- `NNNN` es 1 más el mayor número que ya exista en `audio/` con ese prefijo, o `0001` si no hay ninguno. Se calcula mirando la carpeta, no el proyecto, para no reutilizar el nombre de un archivo que el historial de deshacer todavía puede necesitar.
- El `name` de una toma es «Toma N» (en inglés, «Take N»), con N igual a `NNNN` sin ceros a la izquierda. El de un archivo importado es su nombre original sin extensión.
- Un archivo de audio no se modifica nunca después de crearse. Solo se borra en la limpieza al abrir (RF-029).

### 6.3 Archivos de picos (`.peaks`)

Sirven para dibujar la onda sin leer el audio. Hay uno por archivo de audio, con el mismo nombre base. Todos los números son *little-endian*.

| Desplazamiento | Tamaño | Contenido |
|---|---|---|
| 0 | 4 bytes | Texto ASCII `CPK1` |
| 4 | `u32` | Frames por cubo: siempre 256 |
| 8 | `u32` | Número de cubos: `ceil(frames / 256)` |
| 12 | 8 bytes por cubo | Por cada cubo: `f32` mínimo y `f32` máximo de las muestras del cubo |

En un archivo estéreo, el mínimo y el máximo se calculan sobre los dos canales juntos. Si falta un archivo de picos, se vuelve a calcular a partir del audio.

### 6.4 Copia de recuperación

`recovery/project.recovery.carabo` tiene exactamente el formato de `project.carabo`. Su fecha es la fecha de modificación del archivo. Las reglas de cuándo se escribe y se borra están en RF-026 y RF-027.

## 7. Datos de la aplicación

Las rutas las da Tauri a partir del identificador de la aplicación, `com.carabo.app` (07):

| Carpeta | Ruta habitual en Windows | Contenido |
|---|---|---|
| Configuración | `%APPDATA%\com.carabo.app\` | `settings.json`, `session.json` |
| Datos locales | `%LOCALAPPDATA%\com.carabo.app\` | `unsaved\<uuid>\` (proyectos sin guardar), `logs\` |

### 7.1 Ajustes: `settings.json`

```ts
interface Settings {
  settingsVersion: 1;
  language: "es" | "en";
  outputDeviceName: string | null;        // null = predeterminado de Windows
  inputDeviceName: string | null;         // null = predeterminado de Windows
  latencyMsByPair: Record<string, number>; // clave: "<nombre de entrada>|<nombre de salida>"; valor: entero 0..500
  latencyNoticeShownForPairs: string[];    // parejas para las que ya se mostró el aviso de RF-507
  recents: RecentProject[];                // 0..RECENTS_MAX, del más reciente al más antiguo
}

interface RecentProject {
  dir: string;        // ruta absoluta de la carpeta de proyecto
  openedAt: string;   // fecha y hora ISO 8601 en UTC
}
```

- La clave de una pareja usa los nombres reales de los dispositivos en uso, también cuando se eligió «Predeterminado de Windows».
- El retardo se guarda en milisegundos enteros. Un milisegundo son 48 frames: la conversión está en 04 §7.4.
- Valores iniciales: `language` según RF-701; los dos dispositivos, `null`; los demás, vacíos.
- Si el archivo no existe o no se puede leer o validar, se usan los valores iniciales (RF-702).

Ejemplo:

```json
{
  "settingsVersion": 1,
  "language": "es",
  "outputDeviceName": null,
  "inputDeviceName": "Micrófono (USB Audio Device)",
  "latencyMsByPair": {
    "Micrófono (USB Audio Device)|Auriculares (Realtek Audio)": 38
  },
  "latencyNoticeShownForPairs": ["Micrófono (USB Audio Device)|Auriculares (Realtek Audio)"],
  "recents": [
    { "dir": "C:\\Users\\ana\\Documents\\Carabo\\Demo", "openedAt": "2026-11-15T19:40:00Z" }
  ]
}
```

### 7.2 Sesión: `session.json`

Permite detectar un cierre inesperado (RF-027).

```ts
interface Session {
  projectDir: string | null;   // carpeta del proyecto abierto, o null si no hay ninguno
  isTemp: boolean;             // true si es un proyecto que nunca se ha guardado
  cleanExit: boolean;          // false mientras el programa está abierto; true al cerrarse bien
}
```

- Al abrir, crear o guardar como: se escribe con `cleanExit: false` y la carpeta actual.
- Al cerrar el programa correctamente: se escribe con `cleanExit: true`.
- Al arrancar: si existe, `cleanExit` es `false` y `projectDir` contiene `recovery/project.recovery.carabo`, se ofrece la recuperación.

### 7.3 Registro

`logs\carabo.log`. Una línea por entrada: `<fecha y hora ISO 8601 UTC> <NIVEL> <módulo> <mensaje>`, con niveles `ERROR`, `WARN` e `INFO`. Al llegar a 1 MB el archivo pasa a `carabo.1.log` (y el anterior `carabo.1.log` a `carabo.2.log`); se conservan 3 archivos (RNF-20).

## 8. Kits

### 8.1 Orden de funciones

Todos los kits tienen 8 sonidos en el mismo orden de **funciones**. Así, un patrón tiene sentido con cualquier kit y un ritmo hecho sirve para todos.

| Fila | Función (`role`) | Papel en el ritmo |
|---|---|---|
| 0 | `low` | Golpe grave que marca el pulso |
| 1 | `snare` | Golpe seco de respuesta |
| 2 | `clap` | Golpe seco secundario |
| 3 | `hatClosed` | Subdivisión corta |
| 4 | `hatOpen` | Subdivisión larga |
| 5 | `tom` | Relleno |
| 6 | `accent` | Acento |
| 7 | `extra` | Color |

### 8.2 Los tres kits

| Fila | `acoustic` — Batería acústica / Acoustic drums | `electronic` — Batería electrónica / Electronic drums | `percussion` — Percusión / Percussion |
|---|---|---|---|
| 0 | Bombo / Kick | Bombo / Kick | Cajón grave / Cajón low |
| 1 | Caja / Snare | Caja / Snare | Cajón agudo / Cajón high |
| 2 | Aro / Rimshot | Palmada / Clap | Palmas / Handclaps |
| 3 | Charles cerrado / Closed hi-hat | Charles cerrado / Closed hi-hat | Shaker / Shaker |
| 4 | Charles abierto / Open hi-hat | Charles abierto / Open hi-hat | Pandereta / Tambourine |
| 5 | Tom / Tom | Tom / Tom | Bongó / Bongo |
| 6 | Crash / Crash | Platillo / Cymbal | Campana / Bell |
| 7 | Ride / Ride | Cencerro / Cowbell | Claves / Claves |

Grupos de corte: en `acoustic` y `electronic`, las filas 3 y 4 comparten el grupo `"hat"`. `percussion` no tiene ninguno.

### 8.3 Archivos de un kit

```text
assets/kits/acoustic/
├── kit.json
├── low.wav
├── snare.wav
├── clap.wav
├── hat-closed.wav
├── hat-open.wav
├── tom.wav
├── accent.wav
└── extra.wav
```

Cada WAV es mono, a 48 000 Hz, de 16 bits o de coma flotante de 32 bits, y dura como máximo 4 segundos. El nombre del archivo es fijo para cada función.

`kit.json`:

```json
{
  "id": "acoustic",
  "name": { "es": "Batería acústica", "en": "Acoustic drums" },
  "sounds": [
    { "role": "low",       "file": "low.wav",        "name": { "es": "Bombo", "en": "Kick" },                    "chokeGroup": null },
    { "role": "snare",     "file": "snare.wav",      "name": { "es": "Caja", "en": "Snare" },                    "chokeGroup": null },
    { "role": "clap",      "file": "clap.wav",       "name": { "es": "Aro", "en": "Rimshot" },                   "chokeGroup": null },
    { "role": "hatClosed", "file": "hat-closed.wav", "name": { "es": "Charles cerrado", "en": "Closed hi-hat" }, "chokeGroup": "hat" },
    { "role": "hatOpen",   "file": "hat-open.wav",   "name": { "es": "Charles abierto", "en": "Open hi-hat" },   "chokeGroup": "hat" },
    { "role": "tom",       "file": "tom.wav",        "name": { "es": "Tom", "en": "Tom" },                       "chokeGroup": null },
    { "role": "accent",    "file": "accent.wav",     "name": { "es": "Crash", "en": "Crash" },                   "chokeGroup": null },
    { "role": "extra",     "file": "extra.wav",      "name": { "es": "Ride", "en": "Ride" },                     "chokeGroup": null }
  ]
}
```

- `sounds` tiene exactamente 8 elementos, en el orden de §8.1.
- La interfaz lee `kit.json` para los nombres; el motor, para cargar los WAV y los grupos de corte.

### 8.4 Sonidos sustitutos

Mientras no estén elegidos los sonidos definitivos (DEC-33, 10), un script del repositorio (07) genera los 24 archivos WAV de forma determinista. Reglas comunes: mono, 48 000 Hz, 16 bits; pico normalizado a 0,8; fundido de salida lineal en los últimos 5 ms.

- **Ruido**: generador congruencial `estado = (estado · 1664525 + 1013904223) mod 2^32`, `valor = estado / 2^31 − 1`, con semilla `1 + fila + 10 · índiceDelKit` (índices de kit: `acoustic` 0, `electronic` 1, `percussion` 2). «Ruido agudo» es la diferencia entre cada valor y el anterior.
- **Barrido**: seno cuya frecuencia en el instante `t` es `fFin + (fInicio − fFin) · e^(−t/τf)`.
- **Envolvente**: amplitud `e^(−t/τa)`.

| Fila | `acoustic` | `electronic` | `percussion` |
|---|---|---|---|
| 0 `low` | Barrido 160→55 Hz, τf 0,03, τa 0,12; 0,40 s | Barrido 220→40 Hz, τf 0,05, τa 0,25; 0,60 s | Barrido 140→80 Hz, τf 0,02, τa 0,08; 0,30 s |
| 1 `snare` | Seno 190 Hz × 0,5 (τa 0,06) + ruido agudo × 0,7 (τa 0,09); 0,30 s | Seno 220 Hz × 0,5 (τa 0,06) + ruido agudo × 0,7 (τa 0,12); 0,35 s | Seno 320 Hz × 0,5 (τa 0,04) + ruido agudo × 0,6 (τa 0,05); 0,20 s |
| 2 `clap` | Seno 800 Hz × 0,6 (τa 0,02) + ruido agudo × 0,5 (τa 0,01); 0,12 s | Ruido agudo: tres ráfagas en 0, 12 y 24 ms (τa 0,004) y cola desde 36 ms (τa 0,07); 0,30 s | Igual que `electronic`, con cola de τa 0,05; 0,25 s |
| 3 `hatClosed` | Ruido agudo, τa 0,015; 0,08 s | Ruido agudo, τa 0,012; 0,06 s | Ruido agudo con subida lineal de 10 ms y τa 0,04; 0,12 s |
| 4 `hatOpen` | Ruido agudo, τa 0,12; 0,50 s | Ruido agudo, τa 0,10; 0,40 s | Ruido agudo, τa 0,10; 0,35 s |
| 5 `tom` | Barrido 180→110 Hz, τf 0,04, τa 0,15; 0,40 s | Barrido 150→70 Hz, τf 0,04, τa 0,20; 0,45 s | Barrido 400→330 Hz, τf 0,02, τa 0,07; 0,20 s |
| 6 `accent` | Ruido agudo, τa 0,40; 1,50 s | Ruido agudo, τa 0,30; 1,20 s | Senos 2 000 Hz × 0,6 + 2 960 Hz × 0,4, τa 0,30; 1,00 s |
| 7 `extra` | Ruido agudo × 0,4 (τa 0,25) + seno 5 000 Hz × 0,3 (τa 0,20); 0,80 s | Ondas cuadradas 540 Hz × 0,5 + 800 Hz × 0,5, τa 0,12; 0,40 s | Seno 2 500 Hz, τa 0,03; 0,12 s |

Los sonidos definitivos sustituirán a estos archivos conservando los nombres; nada más cambia.

## 9. Ritmos hechos

### 9.1 Archivo `assets/presets/presets.json`

```ts
interface PresetsFile {
  presets: RhythmPreset[];
}

interface RhythmPreset {
  id: string;                                 // identificador estable, en minúsculas
  name: { es: string; en: string };
  timeSignature: TimeSignature;
  bars: 1 | 2;
  rows: Partial<Record<Role, string>>;        // por función: cadena de (bars · stepsPerBar) caracteres
}

type Role = "low" | "snare" | "clap" | "hatClosed" | "hatOpen" | "tom" | "accent" | "extra";
```

- En cada cadena, `x` es un golpe y `-` es un paso vacío. La posición del carácter es el índice del paso.
- Una función que no aparece en `rows` queda vacía.
- Al cargar un ritmo hecho (RF-209), cada `x` se convierte en nivel 2.

Ejemplo de un elemento:

```json
{
  "id": "rock-basic",
  "name": { "es": "Rock básico", "en": "Basic rock" },
  "timeSignature": "4/4",
  "bars": 1,
  "rows": {
    "low":       "x-------x-------",
    "snare":     "----x-------x---",
    "hatClosed": "x-x-x-x-x-x-x-x-"
  }
}
```

### 9.2 Los quince ritmos hechos

Se escriben en `presets.json` en este orden, que es el orden en que se muestran.

**Compás 4/4 (16 pasos por compás)**

| `id` | Nombre ES / EN | Compases | Filas |
|---|---|---|---|
| `rock-basic` | Rock básico / Basic rock | 1 | `low` `x-------x-------` · `snare` `----x-------x---` · `hatClosed` `x-x-x-x-x-x-x-x-` |
| `rock-driving` | Rock a corcheas / Driving rock | 1 | `low` `x-----x-x-x-----` · `snare` `----x-------x---` · `hatClosed` `x-x-x-x-x-x-x-x-` · `accent` `x---------------` |
| `pop` | Pop / Pop | 1 | `low` `x------x--x-----` · `snare` `----x-------x---` · `clap` `----x-------x---` · `hatClosed` `x-x-x-x-x-x-x-x-` |
| `ballad` | Balada / Ballad | 1 | `low` `x---------x-----` · `clap` `----x-------x---` · `hatClosed` `x-x-x-x-x-x-x-x-` |
| `funk` | Funk / Funk | 1 | `low` `x--x--x---x--x--` · `snare` `----x--x-x--x---` · `hatClosed` `xxxxxxxxxxxxxxxx` |
| `disco` | Disco / Disco | 1 | `low` `x---x---x---x---` · `snare` `----x-------x---` · `hatClosed` `x---x---x---x---` · `hatOpen` `--x---x---x---x-` |
| `hiphop` | Hip-hop / Hip-hop | 1 | `low` `x-----x---x--x--` · `snare` `----x-------x---` · `hatClosed` `x-x-x-x-x-x-x-x-` |
| `reggae` | Reggae / Reggae | 1 | `low` `--------x-------` · `clap` `--------x-------` · `hatClosed` `x-x-x-x-x-x-x-x-` |
| `rumba` | Rumba / Rumba | 1 | `low` `x-----x-----x---` · `snare` `----x-----x---x-` · `clap` `--x---x---x---x-` · `hatClosed` `x-x-x-x-x-x-x-x-` |
| `bossa` | Bossa nova / Bossa nova | 2 | `low` `x-----x-x-----x-x-----x-x-----x-` · `clap` `x-----x-----x-------x-----x-----` · `hatClosed` `x-x-x-x-x-x-x-x-x-x-x-x-x-x-x-x-` |
| `country` | Country / Country | 1 | `low` `x-------x-------` · `snare` `----x-------x---` · `hatClosed` `x-xxx-xxx-xxx-xx` |

**Compás 3/4 (12 pasos por compás)**

| `id` | Nombre ES / EN | Compases | Filas |
|---|---|---|---|
| `waltz` | Vals / Waltz | 1 | `low` `x-----------` · `snare` `----x---x---` · `hatClosed` `x-x-x-x-x-x-` |
| `waltz-slow` | Vals lento / Slow waltz | 1 | `low` `x-----------` · `clap` `----x---x---` · `hatClosed` `x---x---x---` |

**Compás 6/8 (12 pasos por compás)**

| `id` | Nombre ES / EN | Compases | Filas |
|---|---|---|---|
| `ballad-68` | Balada 6/8 / 6/8 ballad | 1 | `low` `x---------x-` · `snare` `------x-----` · `hatClosed` `x-x-x-x-x-x-` |
| `blues-68` | Blues 6/8 / 6/8 blues | 1 | `low` `x-----------` · `snare` `------x-----` · `hatClosed` `x---x-x---x-` |

## 10. Instantánea del motor

La instantánea (`EngineSnapshot`) es la descripción completa de lo que debe sonar. La interfaz la construye a partir del proyecto y se la envía al motor cada vez que el documento cambia (04). El motor no conoce clips ni bloques: solo **segmentos** ya resueltos con la regla de tapado, en frames (audio) o en pasos (ritmo).

### 10.1 Tipos

Las claves viajan en JSON con los mismos nombres (`camelCase`).

```ts
interface EngineSnapshot {
  snapshotVersion: 1;
  tempoBpm: number;
  timeSignature: TimeSignature;
  tracks: SnapshotTrack[];         // en el orden de las pistas
  master: MasterStrip;
}

type SnapshotTrack = SnapshotRhythmTrack | SnapshotAudioTrack;

interface SnapshotRhythmTrack {
  id: string;
  type: "rhythm";
  strip: ChannelStrip;             // igual que en el proyecto
  kitId: KitId;
  rows: RowSettings[];             // 8
  patterns: SnapshotPattern[];     // todos los patrones de la pista
  segments: RhythmSegment[];       // sin solapes, ordenados por startStep
}

interface SnapshotPattern {
  id: string;
  lengthSteps: number;             // bars · stepsPerBar
  steps: number[][];               // 8 × lengthSteps, niveles 0..3
}

interface RhythmSegment {
  patternId: string;
  startStep: number;               // paso de la canción donde empieza el segmento
  lengthSteps: number;             // ≥ 1
  phaseSteps: number;              // pasos transcurridos desde el principio de su bloque
}

interface SnapshotAudioTrack {
  id: string;
  type: "audio";
  strip: ChannelStrip;
  segments: AudioSegment[];        // sin solapes, ordenados por startFrame
}

interface AudioSegment {
  file: string;                    // ruta absoluta del archivo de audio
  channels: 1 | 2;
  startFrame: number;              // frame de la canción donde empieza el segmento
  lengthFrames: number;            // ≥ 1
  regionStartFrame: number;        // del clip
  regionLengthFrames: number;      // del clip
  phaseFrames: number;             // frames transcurridos desde el principio de su clip
}
```

Cómo interpreta el motor un segmento:

```text
Ritmo:  en el paso de canción (startStep + s), con 0 ≤ s < lengthSteps,
        suena el paso ((phaseSteps + s) mod pattern.lengthSteps) del patrón.
Audio:  en el frame de canción (startFrame + t), con 0 ≤ t < lengthFrames,
        suena el frame del archivo  regionStartFrame + ((phaseFrames + t) mod regionLengthFrames).
```

### 10.2 Resolución de un carril (regla de tapado)

Esta función convierte los elementos de un carril, que pueden solaparse, en segmentos que no se solapan. Se usa igual para clips (unidad: frames) y para bloques (unidad: pasos), y también para dibujar el carril.

```text
resolveLane(elementos) → segmentos
  Cada elemento tiene: id, inicio, fin (inicio < fin, enteros) y z.

  1. Ordena los elementos por z de mayor a menor.
  2. ocupado = lista vacía de intervalos.
  3. Para cada elemento, en ese orden:
       visible = [inicio, fin) menos la unión de «ocupado»   (cero, uno o varios intervalos)
       Por cada intervalo [a, b) de «visible»: emite el segmento { id, inicio: a, fin: b }.
       Añade [inicio, fin) a «ocupado».
  4. Devuelve los segmentos ordenados por inicio.
```

Ejemplo: A = [0, 400) con z = 1 y B = [100, 300) con z = 2 dan tres segmentos: A [0, 100), B [100, 300) y A [300, 400).

### 10.3 Construcción de la instantánea

```text
buildEngineSnapshot(doc, carpetaDeProyecto, archivosQueFaltan) → EngineSnapshot

Para cada pista de audio:
  1. Descarta los clips cuyo archivo está en «archivosQueFaltan» (RF-028).
  2. Para cada clip: inicio = tickToFrame(startTick); fin = inicio + lengthFrames.
  3. segmentos = resolveLane(clips).
  4. Por cada segmento de un clip:
       file               = carpetaDeProyecto + "/" + source.file, como ruta absoluta
       channels           = source.channels
       startFrame         = segmento.inicio
       lengthFrames       = segmento.fin − segmento.inicio
       regionStartFrame   = clip.regionStartFrame
       regionLengthFrames = clip.regionLengthFrames
       phaseFrames        = segmento.inicio − tickToFrame(clip.startTick)

Para cada pista de ritmo:
  1. Para cada bloque: inicio = barToStep(startBar); fin = barToStep(startBar + lengthBars).
  2. segmentos = resolveLane(bloques).
  3. Por cada segmento de un bloque:
       patternId   = block.patternId
       startStep   = segmento.inicio
       lengthSteps = segmento.fin − segmento.inicio
       phaseSteps  = segmento.inicio − barToStep(block.startBar)
  4. patterns = todos los patrones de la pista, con lengthSteps = bars · stepsPerBar.

strip, rows, kitId, master, tempoBpm y timeSignature se copian tal cual.
```

La instantánea no contiene el estado de vista: la claqueta, el bucle y la posición se envían con sus propios comandos (04).

### 10.4 Ejemplo

Instantánea del proyecto de §5.3, guardado en `C:\Users\ana\Documents\Carabo\Demo` (los `strip`, las `rows` y los `steps` se abrevian con `…` porque son iguales a los del proyecto):

```json
{
  "snapshotVersion": 1,
  "tempoBpm": 100,
  "timeSignature": "4/4",
  "tracks": [
    {
      "id": "trk-1",
      "type": "rhythm",
      "strip": { "…": "igual que en el proyecto" },
      "kitId": "acoustic",
      "rows": [ "…" ],
      "patterns": [ { "id": "pat-2", "lengthSteps": 16, "steps": [ "…" ] } ],
      "segments": [ { "patternId": "pat-2", "startStep": 0, "lengthSteps": 256, "phaseSteps": 0 } ]
    },
    {
      "id": "trk-4",
      "type": "audio",
      "strip": { "…": "igual que en el proyecto" },
      "segments": [
        {
          "file": "C:\\Users\\ana\\Documents\\Carabo\\Demo\\audio\\take-0001.wav",
          "channels": 1,
          "startFrame": 460800,
          "lengthFrames": 573600,
          "regionStartFrame": 2400,
          "regionLengthFrames": 573600,
          "phaseFrames": 0
        }
      ]
    }
  ],
  "master": { "gainDb": 0 }
}
```

### 10.5 Versión 1.1

La 1.1 sube `snapshotVersion` a 2 y añade: `swing` en `SnapshotPattern`; en `AudioSegment`, `gainDb` y los datos de fundido de entrada y de salida; las unidades de efecto nuevas en `strip.effects`; `limiterEnabled` en `master`; y en `RowSettings`, la ruta del sonido propio. El detalle se fijará al planificar la 1.1 (09).
