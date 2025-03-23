pub(crate) use bevy::prelude::*;

// module preludes

// global macro for quick entity getting
#[macro_export]
macro_rules! get_single {
    ($q:expr) => {
        match $q.get_single() {
            Ok(m) => m,
            _ => return,
        }
    };
}

// global enum update sets
#[derive(SystemSet, Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub(crate) enum UpdateSet {
    Tickers,
    RecordInput,
    Update,
}
