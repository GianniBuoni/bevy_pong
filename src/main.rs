use bevy::prelude::*;
use pong_rapier::AppPlugin;

fn main() {
    App::new().add_plugins(AppPlugin).run();
}
