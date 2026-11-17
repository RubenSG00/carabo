#![forbid(unsafe_code)]

pub mod constants;
pub mod error;
pub mod kits;
pub mod snapshot;
pub mod time;

mod audio_pool;
mod engine;
mod io;
mod project;
mod rt;
mod telemetry;

pub use audio_pool::AudioPool;
pub use engine::{Engine, EngineConfig, EngineHandle, PlayRequest, RecordRequest, RecordStarted};
pub use error::EngineError;
pub use io::devices::{ActiveDevices, DeviceInfo, DeviceList};
pub use snapshot::EngineSnapshot;
pub use telemetry::{EngineEvent, RecordEndReason, Telemetry, TelemetryMode, TelemetryState};

/// What the tests of `engine/tests/` need to drive the engine without an audio device
/// (06 §2, 08 §8.1). No other code uses it.
#[doc(hidden)]
pub mod testing {
    use std::sync::Arc;

    use crate::kits::Kits;
    pub use crate::project::{EngineProject, Preparer};
    pub use crate::rt::commands::{CommandProducer, PlayMode, RtCommand};
    pub use crate::rt::processor::Processor;
    pub use crate::telemetry::SharedState;
    use crate::{Engine, EngineConfig, EngineError, EngineHandle};

    /// A new `Processor` in `Stopped` that plays `kits`, the end of its command queue and its
    /// atomic values.
    pub fn processor(kits: Arc<Kits>) -> (Processor, CommandProducer, Arc<SharedState>) {
        let (commands, consumer) = crate::rt::commands::command_queue();
        let shared = Arc::new(SharedState::new());
        let processor = Processor::new(consumer, Arc::clone(&shared), kits);
        (processor, commands, shared)
    }

    /// Starts the engine as `Engine::start` does, as if there were no output device.
    pub fn start_engine_without_output(config: EngineConfig) -> Result<Engine, EngineError> {
        Engine::start_with(config, |_, _| None)
    }

    /// The engine clock (06 §6.2).
    pub fn engine_clock(handle: &EngineHandle) -> u64 {
        handle.engine_clock()
    }
}
