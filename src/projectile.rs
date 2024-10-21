use bevy::prelude::*;
use std::collections::HashMap;

pub struct ProjectilePlugin;
impl Plugin for ProjectilePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, projectiles_spawner);
        app.add_systems(Update, projectiles_move);
        app.add_systems(Update, projectiles_despawn);
    }
}

#[allow(dead_code)]
#[derive(Resource)]
pub struct ProjectileSpawner
{
    pub spawner: Option<HashMap<String, Projectile>>, // path, projectile
}

fn projectiles_spawner(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let mut projectiles_spawner = ProjectileSpawner { spawner: None };
    let spawner = HashMap::from([
        (String::from("default"), Projectile{ damage: 32, velocity: Vec3::new(32.0, 0.0, 0.0), radius: 16.0 })
    ]);
    projectiles_spawner.spawner = Some(spawner);
    commands.insert_resource(projectiles_spawner);
}

#[allow(dead_code)]
#[derive(Component, Clone, Copy)]
pub struct Projectile
{
    pub damage: i32, // unused
    pub velocity: Vec3,
    pub radius: f32, // unused
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
    // let image_handle = projectiles_spawner. 
    // testing values will be removed after adding spawning system
    ProjectileBundle {
        projectile: Projectile {
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

// maybe combine into singular function
pub fn projectiles_despawn(
    mut commands: Commands, 
    windows: Query<&Window>,
    entities: Query<(Entity, &Transform), With<Projectile>>,
){
    let window = windows.single();
    for (entity, transform) in entities.iter(){
        // add better despawns check
        if transform.translation.x > window.width()/2.0
        {
            commands.entity(entity).remove::<ProjectileBundle>();
        }
    }
}
