use bevy::prelude::*;

use crate::states::GameStates;
use crate::phone::PhoneFeedNode;

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
    current_meme_entity: Handle<Image>
}

fn load_meme_assets(
    asset_server: Res<AssetServer>,
    mut meme_collection: ResMut<MemeCollection>,
    mut feed_state: ResMut<FeedState>,
) {
    let handles = vec![
        asset_server.load("car.png"),
    ];

    meme_collection.handles = handles;

    if let Some(first_handle) = meme_collection.handles.get(0) {
        feed_state.current_meme_entity = first_handle.clone();
    }
}

fn handle_feed_scrolling(

) {
    
}

fn update_feed_display(
    phone_feed_query: Query<&mut Node, With<PhoneFeedNode>>,
    feed_state: ResMut<FeedState>,
    meme_collection: Res<MemeCollection>,

) {
    //if let Ok(mut feed_node) = phone_feed_query.get_single() {
    //    *feed_node.Image = feed_state.current_meme_entity.clone();
    //}
}
