use bevy::prelude::*;

pub struct HealthPlugin;
impl Plugin for HealthPlugin 
{
    fn build(&self, app: &mut App) {
        app.add_systems(Update, health_despawn);
    }
}

#[derive(Component)]
pub struct Health {
    pub hp: i32,
}

pub fn health_despawn(
    mut commands: Commands,
    query: Query<(Entity, &Health)>,
) {
    for (entity, health) in & query {
        if  health.hp < 1 {
            commands.entity(entity).despawn();
        }
    }
}
