use std::time::Duration;
use bevy::prelude::*;
use bevy::time::Stopwatch;
use crate::projectile::*;

pub struct TowerPlugin;
impl Plugin for TowerPlugin
{
    fn build(&self, app: &mut App) {
        app.add_systems(Update, tower_update);
        app.add_systems(Update, tower_despawn);
    }
}

#[derive(Component)]
pub struct Tower
{
    pub health: i32,
    pub stopwatch: Stopwatch,
    pub cooldown: Duration,
}

#[derive(Bundle)]
pub struct TowerBundle
{
    pub tower: Tower,
    pub sprite: SpriteBundle,
}

// creating tower from tower_id (not implemented)
pub fn tower_create(
    pos: Vec2,
    image_handle: Handle<Image>,
) -> TowerBundle {
    let translation = Vec3::new(pos.x, pos.y, 0.0);
    TowerBundle {
        tower: Tower {
            health: 100,
            stopwatch: Stopwatch::new(),
            cooldown: Duration::from_secs(4),
        },
        sprite: SpriteBundle {
            texture: image_handle,
            transform: Transform::from_translation(translation),
            ..default()
        }
    }
}

// tower spawn projectile in time intervals
pub fn tower_update(
    time: Res<Time>,
    mut commands: Commands,
    asset_server: Res<AssetServer>, // maybe posible to remove
    projectiles_spawner: Res<ProjectileSpawner>, // should contain sprite handle
    mut query: Query<(&mut Tower, &Transform)>,
){
    for (mut tower, transform) in &mut query
    {
        if tower.stopwatch.elapsed() > tower.cooldown
        {
           if let Some(projectile) = projectiles_spawner.spawner.get("default") {
                let bundle = ProjectileBundle{
                    projectile: projectile.clone(),
                    sprite_bundle: SpriteBundle{
                        transform: Transform::from_translation(transform.translation),
                        texture: asset_server.load("Projectiles/Default.png"),
                        ..default()
                    },
                };
                commands.spawn(bundle);
            };
            tower.stopwatch.reset();
        }
        tower.stopwatch.tick(time.delta());
    }
}

pub fn tower_despawn(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Tower)>,
) {
    for (entity, tower) in &mut query {
        if tower.health < 1 {
            commands.entity(entity).remove::<TowerBundle>();
        }
    }
}
