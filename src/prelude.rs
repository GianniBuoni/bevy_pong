pub(crate) use avian2d::prelude::*;
pub(crate) use bevy::prelude::*;

// module preludes
pub(crate) use crate::gameplay::prelude::*;
pub(crate) use crate::screens::prelude::*;
#[allow(unused_imports)]
pub(crate) use crate::theming::prelude::*;

// global constatns
pub(crate) const GAME_W: f32 = 640.;
pub(crate) const GAME_H: f32 = 360.;

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
pub(crate) use crate::get_single;

#[macro_export]
macro_rules! get_single_mut {
    ($q:expr) => {
        match $q.get_single_mut() {
            Ok(m) => m,
            _ => return,
        }
    };
}

pub(crate) use crate::get_single_mut;

// global enum update sets
#[derive(SystemSet, Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub(crate) enum UpdateSet {
    Tickers,
    RecordInput,
    Update,
}
