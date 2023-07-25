use async_std::channel::Receiver;
use libp2p::Swarm;
use futures::{StreamExt, future::Either};
use libp2p::swarm::{SwarmEvent};
use crate::{Behaviour, Command};

pub async fn run(mut swarm: Swarm<Behaviour>, mut receiver: Receiver<Command>) {
  loop {
    let either = futures::future::select(swarm.next(), receiver.next()).await;

    match either {
      Either::Left((SwarmEvent::ConnectionEstablished { endpoint, .. }, _)) => {
        log::info!("Connected to {}", endpoint.get_remote_address());
    }
      Some(Either::Left(SwarmEvent::ConnectionEstablished { endpoint, .. })) => {
        println!("New connection to {:?}", endpoint.get_remote_address());
      }
        // SwarmEvent::ConnectionEstablished { endpoint, .. } => {
        //     println!("New connection to {:?}", endpoint.get_remote_address());
        // },
        // SwarmEvent::NewListenAddr { listener_id, address } => println!("Listening on {listener_id:?} {address:?}"),
        // SwarmEvent::Behaviour(event) => println!("{event:?}"),
        _ => {}
    }
  }
}