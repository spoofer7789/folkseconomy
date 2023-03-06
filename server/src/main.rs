use libp2p::{identity, PeerId, ping};
use std::error::Error;

#[async_std::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let local_key = identity::Keypair::generate_ed25519();
    let local_peer_id = PeerId::from(local_key.public());
    println!("Local peer id: {:?}", local_peer_id);
    let transport = libp2p::development_transport(local_key).await?;
    let behaviour = ping::Behaviour::new(ping::Config::new().with_keep_alive(true));
    Ok(())
}
// mesh/ star network. They are to all hold hold strings of data.
//create an api in here to take in files.
// 
//  This repo is to store large files & repos.
// Don't work on the p2p scaling right away.
// Use a thrid party cdn for now.