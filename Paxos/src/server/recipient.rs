#![allow(unused)]

use std::net::{SocketAddr, TcpListener, TcpStream};
use std::sync::{Mutex, Arc};
use std::thread;
use std::thread::JoinHandle;
use std::sync::mpsc::{Receiver, Sender};
use std::io::prelude::*;
use std::io::{self, Read};
use std::time;

use crate::node::Node;

pub trait Adopt {
  fn reply(&self, msg: &str, sock: &TcpStream) -> std::io::Result<()> {
    Ok(())
  }
}

pub struct Server {
  port: u16,
  listener: JoinHandle<()>,
  recver: Arc<Mutex<Receiver<(usize, String)>>>,
  sender: Sender<(usize, String)>,
  psocks: Arc<Mutex<Vec<TcpStream>>>,
  handles: Vec<JoinHandle<()>>,
}

impl Server {
  pub fn new(port: u16) -> Server{
    let mut handles = vec![];

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
          Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
            let sleep = time::Duration::from_millis(2);
            thread::sleep(sleep);
            continue;
          }
          Err(e) => {
            println!("server accept error: {}", e);
            continue;
          },
        }
      }
    });

    // set up recver channel
    let (rx, handle) = <Server as Node>::recver(Arc::clone(&client_sock_list));
    handles.push(handle);

    // set up sender channle
    let (tx, handle) = <Server as Node>::sender(Arc::clone(&client_sock_list));
    handles.push(handle);

    Server {
      port: port,
      listener: listener,
      recver: Arc::new(Mutex::new(rx)),
      sender: tx,
      psocks: client_sock_list,
      handles: handles,
    }
  }

  pub fn print_recv(&self) {
    let precver = self.recver.clone();
    thread::spawn(move || {
      let recver = precver.lock().unwrap();
      loop {
        let (idx, msg) = (*recver).recv().unwrap();
        let msg = format!("server recv client {}: {}", idx, msg);
        println!("{}", msg);
      }
    });
  }
}

impl Node for Server {

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