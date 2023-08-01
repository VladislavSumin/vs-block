use std::net::{SocketAddr, UdpSocket};
use std::time::SystemTime;
use bevy::prelude::*;
use bevy_renet::renet::{ConnectionConfig, RenetServer, ServerEvent};
use bevy_renet::renet::transport::{NetcodeServerTransport, ServerAuthentication, ServerConfig};
use bevy_renet::RenetServerPlugin;
use bevy_renet::transport::NetcodeServerPlugin;

fn main() {
    let (server, transport) = new_renet_server();

    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(RenetServerPlugin)
        .add_plugins(NetcodeServerPlugin)
        .insert_resource(server)
        .insert_resource(transport)
        .add_systems(Update, handle_events_system)
        .run();
}


fn new_renet_server() -> (RenetServer, NetcodeServerTransport) {
    let server = RenetServer::new(ConnectionConfig::default());
    let public_addr = "127.0.0.1:5000".parse().unwrap();
    let socket = UdpSocket::bind(public_addr).unwrap();
    let server_config = ServerConfig {
        max_clients: 64,
        protocol_id: 1,
        public_addr,
        authentication: ServerAuthentication::Unsecure,
    };
    let current_time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap();

    let transport = NetcodeServerTransport::new(current_time, server_config, socket).unwrap();

    (server, transport)
}

fn connection_config() -> ConnectionConfig {
    ConnectionConfig {
        available_bytes_per_tick: 1024 * 1024,
        client_channels_config: vec![],
        server_channels_config: vec![],
    }
}

fn handle_events_system(mut server_events: EventReader<ServerEvent>) {
    for event in server_events.iter() {
        match event {
            ServerEvent::ClientConnected { client_id } => {
                println!("Client {client_id} connected");
            }
            ServerEvent::ClientDisconnected { client_id, reason } => {
                println!("Client {client_id} disconnected: {reason}");
            }
        }
    }
}
