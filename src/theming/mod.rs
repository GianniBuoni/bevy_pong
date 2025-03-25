#![allow(dead_code)]
#![allow(unused_imports)]

pub(crate) mod prelude {
    pub(crate) use super::interactions::OnClick;
}

use crate::prelude::*;
mod interactions;
mod ui_components;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(interactions::plugin);
}
