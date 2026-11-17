//! The output stream (06 §13.2). The data callback runs on the output audio thread, so it
//! follows RT-01 to RT-10 (06 §4).

use std::panic::{self, AssertUnwindSafe};
use std::sync::Arc;

use cpal::traits::{DeviceTrait, StreamTrait};
use cpal::{FromSample, SampleFormat, SizedSample, StreamConfig, I24, U24};

use crate::io::devices::default_output_device;
use crate::rt::processor::Processor;
use crate::rt::MAX_BLOCK;
use crate::telemetry::SharedState;

/// An open output stream. It sounds while it exists; dropping it closes it. It is created,
/// kept and dropped on the control thread (06 §3).
pub(crate) struct OutputStream {
    _stream: cpal::Stream,
}

/// Opens Windows' default output device with its default configuration (rate, channels and
/// sample format) and buffer size, and hands `processor` to its audio thread. `None` if there
/// is no output device or it cannot be opened.
pub(crate) fn open_default_output(
    processor: Processor,
    shared: Arc<SharedState>,
) -> Option<OutputStream> {
    let device = default_output_device()?;
    let supported = device.default_output_config().ok()?;
    // TODO(T-080): if the device does not work at 48 000 Hz, convert the frequency at the
    // border (06 §13.4). For now its rate is used as it is.
    let config = supported.config();
    let stream = match supported.sample_format() {
        SampleFormat::F32 => build::<f32>(&device, config, processor, shared),
        SampleFormat::F64 => build::<f64>(&device, config, processor, shared),
        SampleFormat::I8 => build::<i8>(&device, config, processor, shared),
        SampleFormat::I16 => build::<i16>(&device, config, processor, shared),
        SampleFormat::I24 => build::<I24>(&device, config, processor, shared),
        SampleFormat::I32 => build::<i32>(&device, config, processor, shared),
        SampleFormat::I64 => build::<i64>(&device, config, processor, shared),
        SampleFormat::U8 => build::<u8>(&device, config, processor, shared),
        SampleFormat::U16 => build::<u16>(&device, config, processor, shared),
        SampleFormat::U24 => build::<U24>(&device, config, processor, shared),
        SampleFormat::U32 => build::<u32>(&device, config, processor, shared),
        SampleFormat::U64 => build::<u64>(&device, config, processor, shared),
        _ => None,
    }?;
    stream.play().ok()?;
    Some(OutputStream { _stream: stream })
}

/// Builds the stream for samples of type `T`. Each sample is converted from `f32` as it is
/// delivered (06 §13.2).
fn build<T>(
    device: &cpal::Device,
    config: StreamConfig,
    mut processor: Processor,
    shared: Arc<SharedState>,
) -> Option<cpal::Stream>
where
    T: SizedSample + FromSample<f32>,
{
    let channels = usize::from(config.channels);
    // RT-01: the stereo buffer of the processor is reserved here, before the stream starts.
    let mut block = vec![0.0_f32; 2 * MAX_BLOCK].into_boxed_slice();
    let data_callback = move |data: &mut [T], _: &cpal::OutputCallbackInfo| {
        let computed = panic::catch_unwind(AssertUnwindSafe(|| {
            render(&mut processor, &mut block, data, channels);
        }));
        if computed.is_err() {
            // 06 §20: a panic does not leave the call. The block is delivered as silence.
            data.fill(T::EQUILIBRIUM);
            shared.raise_internal_error();
        }
    };
    let error_callback = |_error: cpal::Error| {
        // TODO(T-080): count buffer underruns and overruns as audio dropouts (06 §13.2).
        // TODO(T-084): treat a disconnected device as a lost output (06 §13.5).
    };
    device
        .build_output_stream::<T, _, _>(config, data_callback, error_callback, None)
        .ok()
}

/// Fills `data`, of `channels` interleaved channels, with frames that `processor` computes in
/// pieces of at most `MAX_BLOCK` (06 §13.2). `block` holds `2 · MAX_BLOCK` samples.
fn render<T>(processor: &mut Processor, block: &mut [f32], data: &mut [T], channels: usize)
where
    T: SizedSample + FromSample<f32>,
{
    if channels == 0 {
        data.fill(T::EQUILIBRIUM);
        return;
    }
    for piece in data.chunks_mut(channels * MAX_BLOCK) {
        let frames = piece.len() / channels;
        let Some(stereo) = block.get_mut(..2 * frames) else {
            piece.fill(T::EQUILIBRIUM);
            continue;
        };
        processor.process(stereo, frames);
        distribute(stereo, piece, channels);
    }
}

/// Distributes interleaved stereo frames over the device's channels (06 §13.2): a device with
/// two or more channels gets left and right in its first two channels and silence in the
/// others; a mono device gets `(left + right) / 2`. An incomplete frame at the end is silence.
fn distribute<T>(stereo: &[f32], out: &mut [T], channels: usize)
where
    T: SizedSample + FromSample<f32>,
{
    let mut out_frames = out.chunks_exact_mut(channels);
    for (frame, &[left, right]) in (&mut out_frames).zip(stereo.as_chunks::<2>().0) {
        for (channel, sample) in frame.iter_mut().enumerate() {
            let value = match (channels, channel) {
                (1, _) => (left + right) / 2.0,
                (_, 0) => left,
                (_, 1) => right,
                _ => 0.0,
            };
            *sample = T::from_sample(value);
        }
    }
    out_frames.into_remainder().fill(T::EQUILIBRIUM);
}

#[cfg(test)]
mod tests {
    use super::*;

    const STEREO: [f32; 4] = [0.5, -0.25, 1.0, 0.0];

    #[test]
    fn a_stereo_device_gets_left_and_right() {
        let mut out = [9.0_f32; 4];
        distribute(&STEREO, &mut out, 2);
        assert_eq!(out, STEREO);
    }

    #[test]
    fn a_mono_device_gets_the_mean_of_both_channels() {
        let mut out = [9.0_f32; 2];
        distribute(&STEREO, &mut out, 1);
        assert_eq!(out, [0.125, 0.5]);
    }

    #[test]
    fn channels_after_the_second_are_silent() {
        let mut out = [9.0_f32; 8];
        distribute(&STEREO, &mut out, 4);
        assert_eq!(out, [0.5, -0.25, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0]);
    }

    #[test]
    fn an_incomplete_last_frame_is_silent() {
        let mut out = [9.0_f32; 5];
        distribute(&STEREO, &mut out, 2);
        assert_eq!(out, [0.5, -0.25, 1.0, 0.0, 0.0]);
    }
}
