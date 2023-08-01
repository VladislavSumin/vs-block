use std::net::UdpSocket;
use std::time::SystemTime;
use bevy::prelude::*;
use bevy_renet::renet::{ConnectionConfig, DefaultChannel, RenetClient};
use bevy_renet::renet::transport::{ClientAuthentication, NetcodeClientTransport};
use bevy_renet::RenetClientPlugin;
use bevy_renet::transport::NetcodeClientPlugin;

fn main() {
    let (client, transport) = new_renet_client();
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(RenetClientPlugin)
        .add_plugins(NetcodeClientPlugin)
        .insert_resource(client)
        .insert_resource(transport)
        //.add_systems(Update, send_message_system)
        .run();
}

fn new_renet_client() -> (RenetClient, NetcodeClientTransport) {
    let client = RenetClient::new(ConnectionConfig::default());

    let server_addr = "127.0.0.1:5000".parse().unwrap();
    let socket = UdpSocket::bind("127.0.0.1:0").unwrap();
    let current_time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap();
    let client_id = current_time.as_millis() as u64;
    let authentication = ClientAuthentication::Unsecure {
        client_id,
        protocol_id: 1,
        server_addr,
        user_data: None,
    };

    let transport = NetcodeClientTransport::new(current_time, authentication, socket).unwrap();

    (client, transport)
}

fn connection_config() -> ConnectionConfig {
    ConnectionConfig {
        available_bytes_per_tick: 1024 * 1024,
        client_channels_config: vec![],
        server_channels_config: vec![],
    }
}

fn send_message_system(mut client: ResMut<RenetClient>) {
    // Send a text message to the server
    client.send_message(DefaultChannel::ReliableOrdered, "server message".as_bytes().to_vec());
}