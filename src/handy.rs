use bevy::prelude::*;
use bevy::color::palettes::css::DARK_GRAY;

use crate::states::GameStates;

pub struct HandyPlugin;

impl Plugin for HandyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_handy_ui)
            .add_systems(Update, handle_handy_input.run_if(in_state(GameStates::Erkunden).or(in_state(GameStates::HandyOffen))));
    }
}

#[derive(Component)]
struct HandyRootNode;

fn spawn_handy_ui(mut commands: Commands) {
     commands.spawn((
        Node {
            width: Val::Percent(30.),
            height: Val::Percent(90.),
            align_self: AlignSelf::Center,
            justify_self: JustifySelf::Center,
            ..default()
        },
        
        BackgroundColor(DARK_GRAY.into()),
        HandyRootNode,
        Visibility::Hidden,
        Name::new("Handy"),
    ));
}

fn handle_handy_input(
    input: Res<ButtonInput<KeyCode>>,
    current_state: Res<State<GameStates>>,
    mut next_state: ResMut<NextState<GameStates>>,
    mut query: Query<&mut Visibility, With<HandyRootNode>>,
) {
    if input.just_pressed(KeyCode::KeyH) {
        let mut visibility = query.single_mut();

        match current_state.get() {
            GameStates::Erkunden => {
                next_state.set(GameStates::HandyOffen);
                *visibility = Visibility::Visible;
            }
            GameStates::HandyOffen => {
                next_state.set(GameStates::Erkunden);
                *visibility = Visibility::Hidden;
            }
        }
    }
}

