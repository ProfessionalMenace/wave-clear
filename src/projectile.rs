use bevy::prelude::*;
use bevy::asset::LoadedFolder;

pub struct ProjectilePlugin;
impl Plugin for ProjectilePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, projectiles_move);
        app.add_systems(Update, projectiles_despawn);
    }
}

#[allow(dead_code)]
#[derive(Resource)]
struct ProjectileLoader
{
    handles: Handle<LoadedFolder>,
}

#[allow(dead_code)]
fn load_projectile_textures(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let projectile_loader = ProjectileLoader{
        handles: asset_server.load_folder("assets/Projectiles"),
    };
    commands.insert_resource(projectile_loader);
}

#[allow(dead_code)]
#[derive(Component, Clone, Copy)]
pub struct Projectile
{
    pub damage: i32,
    pub velocity: Vec3,
    pub radius: f32,
}

#[derive(Bundle)]
pub struct ProjectileBundle
{
    pub projectile: Projectile,
    pub sprite_bundle: SpriteBundle,
}

pub fn projectile_create(
    translation: Vec3,
    image_handle: Handle<Image>,
) -> ProjectileBundle{
    ProjectileBundle {
        projectile: Projectile { // testing values will be removed after adding spawning system
            damage: 32,
            velocity: Vec3::new(32.0, 0.0, 0.0),
            radius: 10.0,
            },
        sprite_bundle: SpriteBundle {
            texture: image_handle,
            transform: Transform::from_translation(translation),
            ..default()
        },
    }
}
// update every frame, linear movement
pub fn projectiles_move(
    time: Res<Time>,
    mut query: Query<(&Projectile, &mut Transform)>,
){
    for (projectile, mut transform) in &mut query {
        let displacement = projectile.velocity * time.delta_seconds();
        transform.translation += displacement;
    }
}

pub fn projectiles_despawn(
    mut commands: Commands, 
    windows: Query<&Window>,
    entities: Query<(Entity, &Transform), With<Projectile>>,
){
    let window = windows.single();
    for (entity, transform) in entities.iter(){
        if transform.translation.x > window.width()/2.0 // add better bounds check
        {
            commands.entity(entity).remove::<ProjectileBundle>();
        }
    }
}