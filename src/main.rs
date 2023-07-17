use futures::StreamExt;
use libp2p::{identity, PeerId, ping, Multiaddr};
use libp2p::swarm::{keep_alive, NetworkBehaviour, SwarmBuilder};
use std::error::Error; 
mod network;
#[derive(NetworkBehaviour, Default)]
pub struct Behaviour {
    keep_alive: keep_alive::Behaviour,
    ping: ping::Behaviour
}

pub enum Command {
    Dial(Multiaddr)
}

#[async_std::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Generate a key pair
    let local_key = identity::Keypair::generate_ed25519();
    println!("{local_key:?}");
    let local_peer_id = PeerId::from(local_key.public());
    println!("Local peer id: {local_peer_id:?}");

    // Construct a transport 
    let transport = libp2p::development_transport(local_key).await?;
    println!("Transport created");
    let behaviour = Behaviour::default();


    let mut swarm = SwarmBuilder::with_async_std_executor(transport, behaviour, local_peer_id).build();


    swarm.listen_on("/ip4/0.0.0.0/tcp/0".parse()?)?;


    if let Some(addr) = std::env::args().nth(1) {
       let remote: Multiaddr = addr.parse()?;
       swarm.dial(remote)?;
       println!("Dialed {addr}");
    }

    let (sender, rec) = futures::channel::mpsc::channel::<Command>(1);

    network::run(swarm, rec).await;

    Ok(())
}
