use crate::events::WestinyEvent;
use amethyst::prelude::*;
use amethyst::core::{Time, Transform};
use westiny_server::resources::{ClientRegistry, NetworkIdSupplier};
use crate::resources::{ProjectileCollisions, Collisions};

use log::info;
use std::path::PathBuf;
use derive_new::new;
use westiny_common::resources::map::build_map;
use westiny_common::components::{NetworkId, Player, Velocity, BoundingCircle, weapon::Weapon, Projectile};

#[derive(new)]
pub struct ServerState {
    resources: PathBuf,
}

impl ServerState {
    fn place_objects(&self, world: &mut World) {
        const ONLY_VALID_SEED: u64 = 0;

        build_map(world, ONLY_VALID_SEED, &self.resources.join("map"))
            .expect("Map could not be created");
    }
}

fn log_fps(time: &Time) {
    if time.frame_number() % 60 == 0 {
        // Note: this is not an average, calculated from the last frame delta.
        info!("FPS: {}", 1.0 / time.delta_real_seconds());
    }
}


fn log_clients(time: &Time, registry: &ClientRegistry) {
    if time.frame_number() % 600 == 0 {
        log::info!("Number of players online: {}", registry.client_count());
        log::debug!("{}", &registry);
    }
}


impl State<GameData<'static, 'static>, WestinyEvent> for ServerState {
    fn on_start(&mut self, data: StateData<'_, GameData<'static, 'static>>) {
        data.world.insert(ClientRegistry::new(16));
        data.world.insert(NetworkIdSupplier::new());
        data.world.insert(Collisions::default());
        data.world.insert(ProjectileCollisions::default());

        data.world.register::<NetworkId>();
        data.world.register::<Player>();
        data.world.register::<Velocity>();
        data.world.register::<BoundingCircle>();
        data.world.register::<Weapon>();
        data.world.register::<Transform>();
        data.world.register::<Projectile>();
        self.place_objects(data.world);
    }

    fn update(&mut self, data: StateData<'_, GameData<'static, 'static>>) -> Trans<GameData<'static, 'static>, WestinyEvent> {
        let time = *data.world.fetch::<Time>();
        log_fps(&time);
        data.data.update(&data.world);
        log_clients(&time, &data.world.fetch::<ClientRegistry>());
        Trans::None
    }
}
