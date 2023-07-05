use libp2p::Swarm;
use futures::StreamExt;
use libp2p::swarm::{SwarmEvent};
use crate::Behaviour;

pub async fn run(mut swarm: Swarm<Behaviour>) {
  loop {
    match swarm.select_next_some().await {
        SwarmEvent::ConnectionEstablished { endpoint, .. } => {
            println!("New connection to {:?}", endpoint.get_remote_address());
        },
        SwarmEvent::NewListenAddr { listener_id, address } => println!("Listening on {listener_id:?} {address:?}"),
        SwarmEvent::Behaviour(event) => println!("{event:?}"),
        _ => {}
    }
  }
}