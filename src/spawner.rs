use bevy::prelude::*;
use std::collections::HashMap;

#[allow(dead_code)]
#[derive(Resource)]
pub struct Spawner<T> {
    default: T,
    spawner: HashMap<String, T>,
}

#[allow(dead_code)]
impl<T> Spawner<T>
where
    T: Clone
{
    pub fn get(self: &Spawner<T>, name: &String) -> &T {
        match self.spawner.get(name) {
            None => &self.default,
            Some(result) => result,
        }
    }

    pub fn new(default: &T) -> Self {
        Spawner {
            default: default.clone(),
            spawner: HashMap::new(),
        }
    }
}
