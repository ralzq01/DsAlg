#![allow(unused)]

use std::net::{SocketAddr, TcpListener, TcpStream};
use std::sync::{Mutex, Arc};
use std::thread;
use std::thread::JoinHandle;
use std::sync::mpsc::Receiver;

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
  recver: Arc<Mutex<Receiver<String>>>,
  sock_list: Arc<Mutex<Vec<TcpStream>>>,
}

impl Server {
  pub fn new(port: u16) -> Server{

    let listen = TcpListener::bind(SocketAddr::from(([127, 0, 0, 1], port))).unwrap();
    listen.set_nonblocking(true)
          .expect("server can't set nonblocking tcp stream");
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

    let rx = <Server as Node>::recv(Arc::clone(&client_sock_list));

    Server {
      port: port,
      listener: listener,
      recver: Arc::new(Mutex::new(rx)),
      sock_list: client_sock_list,
    }
  }

  pub fn print_recv(&self) {
    let precver = self.recver.clone();
    thread::spawn(move || {
      let recver = precver.lock().unwrap();
      loop {
        println!("{}", (*recver).recv().unwrap());
      }
    });
  }
}

impl Node for Server {}