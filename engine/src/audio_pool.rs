//! The audio pool: the audio files loaded whole into memory (06 §9.1).

/// The audio loaded into memory, shared by the control thread, the loader and the export
/// (06 §9.1).
// TODO(T-044): its entries, the loader thread and the loading of each file (06 §9.1).
#[derive(Debug, Clone, Default)]
pub struct AudioPool {}
