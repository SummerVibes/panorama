use gossip::{UpdateHandler, Update, Peer, GossipService};

pub struct MyUpdateHandler;

impl UpdateHandler for MyUpdateHandler {
    fn on_update(&self, update: Update) {
        let _string_message = String::from_utf8(update.content().to_vec()).unwrap();
        println!("{}",_string_message);
        // do something with the message...
    }
}

fn main() {
    // local machine IP and port for listening
    let address = "127.0.0.1:9000";

    // existing peer(s) in the network
    let existing_peers = || Some(vec![ Peer::new("127.0.0.1:9001".to_owned()) ]);

    // create and start the service
    let mut gossip_service = GossipService::new_with_defaults(address.parse().unwrap());
    gossip_service.start(Box::new(existing_peers), Box::new(MyUpdateHandler)).unwrap();

    // submit a message
    gossip_service.submit("Some random message".as_bytes().to_vec()).unwrap();
    // shutdown the gossip protocol on exit
    //gossip_service.shutdown();
}