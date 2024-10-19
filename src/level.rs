use std::string;
use bevy::prelude::*;

pub struct LevelPlugin;
impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
    }
}

#[derive(Resource)]
pub struct Level
{
    tilesize: u32,
    background_img: String,
}

fn setup(mut commands: Commands,
    asset_server: Res<AssetServer>,
){
    commands.spawn(
        SpriteBundle {
            texture: asset_server.load("background.png"),
            ..default()
        }
    );
}