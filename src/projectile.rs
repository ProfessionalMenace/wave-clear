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

#[derive(Resource)]
pub struct ProjectileSpawner {
    pub spawner: HashMap<String, ProjectileBundle>,
}

fn projectiles_spawner(mut commands: Commands, asset_server: Res<AssetServer>) {
    let projectiles_spawner = ProjectileSpawner {
        spawner: HashMap::from([(
            String::from("default"),
            ProjectileBundle {
                projectile: Projectile {
                    damage: 32,
                    velocity: Vec3::new(32.0, 0.0, 0.0),
                    radius: 16.0,
                },
                sprite: SpriteBundle {
                    texture: asset_server.load("Projectiles/Default.png"),
                    ..default()
                },
            },
        )]), 
    };
    commands.insert_resource(projectiles_spawner);
}

#[allow(dead_code)]
#[derive(Component, Clone, Copy)]
pub struct Projectile {
    pub damage: i32, // unused
    pub velocity: Vec3,
    pub radius: f32, // unused
}

#[derive(Bundle, Clone)]
pub struct ProjectileBundle {
    pub projectile: Projectile,
    pub sprite: SpriteBundle,
}

// update every frame, linear movement
pub fn projectiles_move(time: Res<Time>, mut query: Query<(&Projectile, &mut Transform)>) {
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
) {
    let window = windows.single();
    for (entity, transform) in entities.iter() {
        // add better despawns check
        if transform.translation.x > window.width() / 2.0 {
            commands.entity(entity).remove::<ProjectileBundle>();
        }
    }
}
