use std::io::prelude::*;
use std::net::TcpStream;
use std::net::SocketAddr;

use crate::node::Node;

pub trait Propose {
  /// propose a method and ticket to all servers
  fn propose(&self, method: &str, ticket: u32) {
    
  }
}

pub struct Client {
  port: u32,  // self port
  server_sock_list: Vec<TcpStream>,
  server_port_list: Vec<u32>,
  tickets: u32,
}

impl Client {
  pub fn new(port: u32, server_port_list: &Vec<u32>) -> Client {
    let mut server_sock_list = vec![];
    let mut fail_port_idx = vec![];
    for idx in (0..server_port_list.len()) {
      let addrs = SocketAddr::from(([127, 0, 0, 1], server_port_list[idx]));
      if let mut Ok(sock) = TcpStream::connect(&addrs) {
        server_sock_list.push(sock);
      } else {
        println!("client port: {} can't connect with server port: {}", port, server_port);
        fail_port_idx.push(idx);
      }
    }
  }
}

impl Node for Client;

