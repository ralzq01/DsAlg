
mod client;
use crate::client::proposer::Client;

mod server;
use crate::server::recipient::Server;

mod node;
use crate::node::Node;

fn main() {
    let server_num = 2;
    let client_num = 2;
    let server_port_list : Vec<u16> = (8020..8020+server_num).collect();
    let mut server_list = vec![];
    let mut client_list = vec![];

    // first create server
    for port in &server_port_list {
        let server = Server::new(*port);
        server_list.push(server);
    }
    println!("all servers have been initialized successfully");
    
    // create client
    for id in 0..client_num {
        let client = Client::new(id, &server_port_list);
        client_list.push(client);
    }

    // test for sending message
    for server in &server_list {
        server.print_recv();
    }
    for client in &client_list {
        let msg = format!("client {} broadcasts", client.id);
        client.broadcast(msg.as_str());
    }

    loop {}
}
