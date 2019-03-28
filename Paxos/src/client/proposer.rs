#![allow(unused)]

use std::io::prelude::*;
use std::net::TcpStream;
use std::net::SocketAddr;
use std::sync::{Mutex, Arc};

#[path = "../node.rs"]
mod node;
use self::node::Node;

pub trait Propose {
  /// propose a method and ticket to all servers
  fn propose(&self, method: &str, ticket: u32) -> std::io::Result<()> {
    Ok(())
  }
}

pub struct Client {
  id: u32,
  psocks: Arc<Mutex<Vec<TcpStream>>>,
  server_port_list: Vec<u16>,
  tickets: u32,
}

impl Client {
  pub fn new(id: u32, server_port_list: &Vec<u16>) -> Client {
    let mut server_sock_list = vec![];
    let mut fail_port_idx = vec![];
    for idx in 0..server_port_list.len() {
      let addr = SocketAddr::from(([127, 0, 0, 1], server_port_list[idx]));
      if let Ok(sock) = TcpStream::connect(addr) {
        sock.set_nonblocking(true)
            .expect("client can't set nonblocking tcp stream");
        server_sock_list.push(sock);
      } else {
        println!("client can't connect with server port: {}", &server_port_list[idx]);
        fail_port_idx.push(idx);
      }
    }
    // remove the failed nodes on server prot list
    let mut server_port_list = server_port_list.clone();
    if fail_port_idx.len() > 0 {
      println!("warning: client {} can't connect {} servers", &id, fail_port_idx.len());
    }
    while let Some(fail_idx) =  fail_port_idx.pop() {
      server_port_list.remove(fail_idx);
    }
    Client {
      id: id,
      psocks: Arc::new(Mutex::new(server_sock_list)),
      server_port_list: server_port_list,
      tickets: 0,
    }
  }
}

impl Node for Client {}

