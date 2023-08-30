use bevy::{ecs::query::WorldQuery, prelude::*};
use wag_core::Players;

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
    pub fn get_players(&self) -> (Entity, Entity) {
        let players = self.world().resource::<Players>();
        let p1 = players.one;
        let p2 = players.two;
        // TODO: Migration to bevy 0.11.2 broke this
        // drop(players);
        (p1, p2)
    }

    // TODO: Functions to make querying the world a little less painful.
}
