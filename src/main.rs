use bevy::prelude::*;
mod inputs;
mod tower;
mod projectile;

use crate::projectile::ProjectilePlugin;
use crate::tower::TowerPlugin;
use crate::inputs::InputHandlePlugin;

fn main() { 
    let mut app = App::new();
    app.add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()));
    app.add_plugins(InputHandlePlugin);
    app.add_plugins(ProjectilePlugin);
    app.add_plugins(TowerPlugin);
    app.add_systems(Startup, scene_setup);
    app.run();
}

fn scene_setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>
) {
    let camera_bundle = Camera2dBundle::default();
    commands.spawn(camera_bundle);
    commands.spawn(
        SpriteBundle {
            texture: asset_server.load("background.png"),
            ..default()
        }
    );
}
