use bevy::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player);
    }
}

#[derive(Component)]
pub struct Player;

fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>
) {

    commands.spawn((
        Transform::from_xyz(10., 10., 10.),
        Player,
        Sprite {
            image: asset_server.load("player.png"),
            custom_size: Some(Vec2::new(400., 400.))
            ,..default()
        },
        Name::new("Player"),
    ));
}
