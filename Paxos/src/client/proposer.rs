use std::io::prelude::*;
use std::net::TcpStream;

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
}


impl Node for Client;



impl Propose for Client {
  fn propose
}