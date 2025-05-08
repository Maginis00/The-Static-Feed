use bevy::prelude::*;

use crate::states::GameStates;
use crate::phone::PhoneRootNode;

pub struct FeedPlugin;

impl Plugin for FeedPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MemeCollection>()
            .init_resource::<FeedState>()
            .add_systems(Startup, load_meme_assets)
            .add_systems(Update, handle_feed_scrolling.run_if(in_state(GameStates::PhoneOpen)))
            .add_systems(Update, update_feed_display.run_if(in_state(GameStates::PhoneOpen)))
        ;
    }
}

#[derive(Resource, Default)]
struct MemeCollection {
    handles: Vec<Handle<Image>>
}

#[derive(Resource, Default)]
pub struct FeedState {
    current_meme_entity: Option<Entity>
}

fn load_meme_assets(
    asset_server: Res<AssetServer>,
    meme_collection: Res<MemeCollection>
) {
    
}

fn handle_feed_scrolling(

) {
    
}

fn update_feed_display(
    mut commands: Commands,
    phone_query: Query<Entity, With<PhoneRootNode>>,
    feed_state: ResMut<FeedState>,
    meme_collection: Res<MemeCollection>,

) {
    if let Ok(phone_entity) = phone_query.get_single() {
        
        
    }
}
