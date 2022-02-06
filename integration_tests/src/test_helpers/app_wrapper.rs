use bevy::{ecs::query::WorldQuery, prelude::*};

pub struct AppWrapper {
    app: App,
}
impl AppWrapper {
    pub fn new(app: App) -> Self {
        Self { app }
    }

    pub fn query<Q: WorldQuery>(&mut self) -> QueryState<Q> {
        self.app.world.query::<Q>()
    }
    pub fn world(&self) -> &World {
        &self.app.world
    }

    // TODO: Functions to make querying the world a little less painful.
}
