#![allow(unused)]

use std::net::{SocketAddr, TcpListener, TcpStream};
use std::sync::{Mutex, Arc};
use std::thread;
use std::thread::JoinHandle;

#[path = "../node.rs"]
mod node;
use self::node::Node;

pub trait Adopt {
  fn reply(&self, msg: &str, sock: &TcpStream) -> std::io::Result<()> {
    Ok(())
  }
}

pub struct Server {
  port: u16,
  listener: JoinHandle<()>,
  client_sock_list: Arc<Mutex<Vec<TcpStream>>>,
}

impl Server {
  pub fn new(port: u16) -> Server{
    let listen = TcpListener::bind(SocketAddr::from(([127, 0, 0, 1], port))).unwrap();
    println!("server has start listening on {}", port);
    let client_sock_list = Arc::new(Mutex::new(vec![]));
    let sock_list = Arc::clone(&client_sock_list);
    let listener = thread::spawn(move || {
      for stream in listen.incoming() {
        match stream {
          Ok(stream) => {
            println!("detect connections");
            let mut lists = sock_list.lock().unwrap();
            (*lists).push(stream);
          }
          Err(e) => {
            //println!("{}", e);
            continue;
          }
        }
      }
    });
    Server {
      port: port,
      listener: listener,
      client_sock_list: client_sock_list,
    }
  }
}

impl Node for Server {}