use std::error::Error;

use libp2p::{identity, Multiaddr, PeerId};
use libp2p::core::muxing::StreamMuxerBox;
use libp2p::core::transport::Boxed;
use libp2p::futures::StreamExt;
use libp2p::ping::{Ping, PingConfig};
use libp2p::swarm::{Swarm, SwarmEvent};
use libp2p::mdns::{Mdns, MdnsConfig, MdnsEvent};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let new_key = identity::Keypair::generate_ed25519();
    let new_peer_id = PeerId::from(new_key.public());

    println!("local peer id: {:?}", new_peer_id);

    let transport: Boxed<(PeerId, StreamMuxerBox)> = libp2p::development_transport(new_key).await?;

    // let behaviour = Ping::new(PingConfig::new().with_keep_alive(true));
    let behaviour = Mdns::new(MdnsConfig::default()).await?;

    let mut swarm = Swarm::new(transport, behaviour, new_peer_id);
    swarm.listen_on("/ip4/0.0.0.0/tcp/0".parse()?)?;

    // if let Some(remote_peer) = std::env::args().nth(1) {
    //     let remote_peer_multiaddr: Multiaddr = remote_peer.parse()?;
    //     println!("Contacted remote peer: {:?}", remote_peer);
    //     swarm.dial(remote_peer_multiaddr)?;
    // }

    loop {
        match swarm.select_next_some().await {
            SwarmEvent::NewListenAddr { address, .. } => {
                println!("Listening on local address {:?}", address)
            }
            SwarmEvent::Behaviour(MdnsEvent::Discovered(peers)) => {
                for (peer, addr) in peers {
                    println!("discovered {} {}", peer, addr);
                }
            }
            SwarmEvent::Behaviour(MdnsEvent::Expired(expired)) => {
                for (peer, addr) in expired {
                    println!("expired {} {}", peer, addr);
                }
            }
            _ => {
                println!("Unknown.")
            }
        }
    }
}
