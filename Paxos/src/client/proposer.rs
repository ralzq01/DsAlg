#![allow(unused)]

use std::io::prelude::*;
use std::net::TcpStream;
use std::net::SocketAddr;
use std::sync::{Mutex, Arc};
use std::sync::mpsc::{Receiver, Sender};
use std::thread;
use std::thread::JoinHandle;
use std::io::prelude::*;

use crate::node::Node;

pub trait Propose {
  /// propose a method and ticket to all servers
  fn propose(&self, method: &str, ticket: u32) -> std::io::Result<()> {
    Ok(())
  }
}

pub struct Client {
  pub id: u32,
  psocks: Arc<Mutex<Vec<TcpStream>>>,
  ports: Vec<u16>,
  recver: Receiver<(usize, String)>,
  sender: Sender<(usize, String)>,
  tickets: u32,
  handles: Vec<JoinHandle<()>>,
}

impl Client {
  pub fn new(id: u32, server_port_list: &Vec<u16>) -> Client {
    let mut server_sock_list = vec![];
    let mut fail_port_idx = vec![];
    let mut handles = vec![];

    // connect to server
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

    // set up recv channels
    let psocks = Arc::new(Mutex::new(server_sock_list));
    let (rx, handle) = <Client as Node>::recver(Arc::clone(&psocks));
    handles.push(handle);

    // set up send channels
    let (tx, handle) = <Client as Node>::sender(Arc::clone(&psocks));
    handles.push(handle);

    Client {
      id: id,
      psocks: psocks,
      ports: server_port_list,
      recver: rx,
      sender: tx,
      tickets: 0,
      handles: handles,
    }
  }
}

impl Node for Client {
  fn get_connection_num(&self) -> usize {
    let socks = self.psocks.lock().unwrap();
    (*socks).len()
  }

  fn send(&self, idx: usize, msg: &str) {
    self.sender.send((idx, msg.to_string()));
  }

  fn broadcast(&self, msg: &str) {
    let num = self.get_connection_num();
    for idx in 0..num {
      self.send(idx, msg);
    }
  }
}

