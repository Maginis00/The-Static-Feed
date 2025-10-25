use bevy::prelude::*;
use bevy::color::palettes::css::DARK_GRAY;
use bevy::color::palettes::css::WHITE;

use crate::states::GameStates;

pub struct PhonePlugin;

impl Plugin for PhonePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_phone_ui)
            .add_systems(Update, handle_phone_input.run_if(in_state(GameStates::Explore).or(in_state(GameStates::PhoneOpen))));
    }
}

#[derive(Component)]
pub struct PhoneRootNode;

#[derive(Component)]
pub struct PhoneFeedNode;

fn spawn_phone_ui (
    mut commands: Commands,
)
{
     commands.spawn((
        Node {
            width: Val::Percent(30.),
            height: Val::Percent(90.),
            align_self: AlignSelf::Center,
            justify_self: JustifySelf::Center,
            ..default()
        },
        
        BackgroundColor(DARK_GRAY.into()),
        PhoneRootNode,
        Visibility::Hidden,
        Name::new("Phone")
    )).with_child((
        Node {
            width: Val::Percent(100.),
            height: Val::Percent(60.),
            align_self: AlignSelf::Center,
            ..default()
        },

        BackgroundColor(WHITE.into()),
        Name::new("APP child"),
        PhoneFeedNode,
    ));
}

fn handle_phone_input(
    input: Res<ButtonInput<KeyCode>>,
    current_state: Res<State<GameStates>>,
    mut next_state: ResMut<NextState<GameStates>>,
    mut query: Query<&mut Visibility, With<PhoneRootNode>>,
) {
    if input.just_pressed(KeyCode::KeyH) {
        let mut visibility = query.single_mut();

        match current_state.get() {
            GameStates::Explore => {
                next_state.set(GameStates::PhoneOpen);
                *visibility = Visibility::Visible;
            }
            GameStates::PhoneOpen => {
                next_state.set(GameStates::Explore);
                *visibility = Visibility::Hidden;
            }
        }
    }
}

