use amethyst::utils::application_root_dir;
use amethyst::{GameDataBuilder, CoreApplication};
use amethyst::network::simulation::laminar::{LaminarNetworkBundle, LaminarSocket, LaminarConfig};
use crate::utilities::*;
use std::net::{IpAddr, SocketAddr};
use std::str::FromStr;
use std::time::Duration;
use amethyst::core::frame_limiter::FrameRateLimitStrategy;
use westiny_common::{
    resources::ServerAddress,
    events::{WestinyEvent, WestinyEventReader},
};
use westiny_server::systems as srv_systems;

mod systems;
mod entities;
mod resources;
mod states;
mod utilities;

#[cfg(test)]
mod test_helpers;

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let app_root = application_root_dir()?;
    let resources_dir = app_root.join("resources");

    let server_port: u16 = {
        let ron_path = resources_dir.join("server_network.ron");
        read_ron::<ServerAddress>(&ron_path)
            .map(|addr| addr.address.port())
            .unwrap_or_else(|err| {
                let srv_port = ServerAddress::default().address.port();
                log::warn!("Failed to read server network configuration file: {}, error: [{}] \
                Using default server port ({})",
                           ron_path.as_os_str().to_str().unwrap(),
                           err,
                           srv_port);
                srv_port
            })
    };
    let socket_address = SocketAddr::new(IpAddr::from_str("0.0.0.0").unwrap(), server_port);
    log::info!("Start listening on {}", socket_address);

    let laminar_config = {
        let mut conf = LaminarConfig::default();
        // send heartbeat in every 3 seconds
        conf.heartbeat_interval = Some(Duration::from_secs(3));
        conf
    };
    let socket = LaminarSocket::bind_with_config(socket_address, laminar_config)?;

    let game_data = GameDataBuilder::default()
        .with_bundle(LaminarNetworkBundle::new(Some(socket)))?
        .with(srv_systems::EntityStateBroadcasterSystem, "entity_state_broadcaster", &[])
        .with_system_desc(srv_systems::NetworkMessageReceiverSystemDesc::default(), "msg_receiver", &[])
        .with_system_desc(srv_systems::ClientIntroductionSystemDesc::default(), "player_spawn", &["msg_receiver"])
        .with_system_desc(srv_systems::CommandTransformerSystemDesc::default(), "command_transformer", &["msg_receiver"])
        .with(srv_systems::PlayerMovementSystem, "player_movement", &["command_transformer"])
        .with(srv_systems::PhysicsSystem, "physics", &["player_movement"])
        .with(srv_systems::CollisionSystem, "collision", &["physics"])
        .with(srv_systems::ProjectileCollisionSystem, "projectile_collision", &["physics"])
        .with(srv_systems::ProjectileCollisionHandler, "projectile_collision_handler", &["projectile_collision"])
        .with(srv_systems::CollisionHandlerForObstacles, "collision_handler", &["collision"])
        .with(srv_systems::ShooterSystem, "shooter", &["command_transformer"])
        .with_system_desc(srv_systems::EntityDeleteBroadcasterSystemDesc::default(), "delete_broadcaster", &["collision_handler"])
        ;

    let frame_limit = 60;

    let mut game =
        CoreApplication::<_, WestinyEvent, WestinyEventReader>::build(
            resources_dir.clone(),
            states::server_states::ServerState::new(resources_dir),
        )?
        .with_frame_limit(
            FrameRateLimitStrategy::SleepAndYield(Duration::from_millis(2)),
            frame_limit
        )
        .build(game_data)?;

    log::info!("Starting server");
    game.run();
    Ok(())
}
