use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

mod player;
mod states;
mod phone;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_state::<states::GameStates>()
        .add_plugins(player::PlayerPlugin)
        .add_plugins(phone::PhonePlugin)
        .add_plugins(WorldInspectorPlugin::new())
        .add_systems(Startup, setup_camera)
        .run();
}

fn setup_camera(
    mut commands: Commands
) {
    commands.spawn(Camera2d);
}
