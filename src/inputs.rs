use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use crate::tower::*;
use crate::spawner::Spawner;

pub struct InputHandlePlugin;
impl Plugin for InputHandlePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, mouse_button_input);
    }
}

// Function for adding basic functionality
fn mouse_button_input(
    mut commands: Commands,
    tower_spawner: Res<Spawner<TowerBundle>>,
    buttons: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window, With<PrimaryWindow>>, // input detection
    camera_query: Query<(&Camera, &GlobalTransform)>, // camera to viewport
) {
    let window = windows.single();
    let (camera, camera_transform) = camera_query.single();

    let Some(cursor_position) = window.cursor_position() else {
        return;
    };

    let Some(point) = camera.viewport_to_world_2d(camera_transform, cursor_position) else {
        return;
    };

    let tower_bundle = tower_spawner.get(&"default".to_owned());
    if buttons.just_released(MouseButton::Left) {
        println!("Tower at {:?}", point);
        commands.spawn(tower_bundle.clone());
    }
}
