use libp2p::{identity, PeerId};
use std::error::Error; 

#[async_std::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Generate a key pair
    let local_key = identity::Keypair::generate_ed25519();
    println!("{local_key:?}");
    let local_peer_id = PeerId::from(local_key.public());
    println!("Local peer id: {local_peer_id:?}");

    // Construct a transport 
    let transport = libp2p::development_transport(local_key);
    println!("Transport created");

    Ok(())
}
