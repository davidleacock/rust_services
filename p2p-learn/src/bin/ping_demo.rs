use std::error::Error;
use std::time::Duration;

use futures::StreamExt;
use libp2p::{Multiaddr, ping, Swarm, swarm};
use libp2p::ping::Behaviour;
use libp2p::swarm::SwarmEvent;
use tracing_subscriber::EnvFilter;

#[async_std::main]
async fn main() -> Result<(), Box<dyn Error>> {
    tracing_subscriber::fmt().with_env_filter(EnvFilter::from_default_env()).init();

    let config = libp2p::ping::Config::new().with_interval(Duration::from_secs(5));

    // TODO how to configure the keep alive?  Even though the ping is happening the server still cuts after 60s

    // The `Swarm` drives both a `Transport` and `Behaviour` forward, passing commands to
    // and from each other
    let mut swarm: Swarm<Behaviour> = libp2p::SwarmBuilder::with_new_identity()
        .with_async_std()
        .with_tcp(                          // TCP Transport (how to send bytes)
            libp2p::tcp::Config::default(), // establish connection
            libp2p::tls::Config::new,       // encrypt connection
            libp2p::yamux::Config::default, // run one or more streams on connect
        )?
        .with_behaviour(|| ping::Behaviour::new(config))?  // Behaviour defines what and whom to send
        .with_swarm_config(|cfg| cfg.with_idle_connection_timeout(Duration::from_secs(60)))
        .build(); // Now we have defined Transport and Behaviour, build the swarm to connect the two


    // Tell swarm to listen on all interfaces and a random, OS-assigned port
    swarm.listen_on("/ip4/0.0.0.0/tcp/0".parse()?)?;

    // If second command line argument provided, connect to the peer identified by the multi-address
    if let Some(addr) = std::env::args().nth(1) {
        let remote: Multiaddr = addr.parse()?;
        swarm.dial(remote)?;
        println!("Connecting to {addr}")
    }

    // Continuously polling the Swarm, listen for incoming connects and establish outgoing
    // connections if address specified
    loop {
        match swarm.select_next_some().await {
            SwarmEvent::NewListenAddr { address, .. } =>
                println!("Listening on address: {address:?}"),
            SwarmEvent::Behaviour(event) => {
                println!("Event: {event:?}")
            }
            SwarmEvent::ConnectionEstablished { peer_id, .. } => {
                println!("ConnectionEstablished: peer_id {peer_id:?}")
            }
            SwarmEvent::ConnectionClosed { peer_id, .. } => {
                println!("ConnectionClosed: peer_id {peer_id:?}")
            }
            SwarmEvent::Dialing { peer_id, .. } => {
                println!("Dialing: peer_id {peer_id:?}")
            }
            SwarmEvent::IncomingConnection { local_addr, .. } => {
                println!("IncomingConnection: local_addr {local_addr:?}")
            }
            _ => {}
        }
    }
}