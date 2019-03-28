use std::io::prelude::*;
use std::io::{self, Read};
use std::net::TcpStream;
use std::thread;
use std::time;
use std::sync::mpsc::channel;
use std::sync::mpsc::Receiver;
use std::sync::{Mutex, Arc};

use rand::prelude::*;

pub trait Node {
  fn send(&self, sock: &'static mut TcpStream, msg: &str) {
    let msg = msg.to_string();
    let handle = thread::spawn(move || {
      let mut rng = rand::thread_rng();
      let random_time: u64 = rng.gen();
      let sleep_time = time::Duration::from_millis(random_time);
      thread::sleep(sleep_time);
      sock.write(msg.as_bytes());
    });
  }

  fn recv(psocks: Arc<Mutex<Vec<TcpStream>>>) -> Receiver<String> {
    let (tx, rx) = channel();
    thread::spawn(move || {
      loop {
        thread::sleep(time::Duration::from_millis(10));
        let socks = psocks.lock().unwrap();
        for mut sock in &(*socks) {
          let mut msg = String::new();
          match sock.read_to_string(&mut msg) {
            Ok(_) => tx.send(msg),
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
              continue;
            }
            Err(e) => panic!("encountered IO error: {}", e),
          };
        }
      }
    });
    rx
  }

  fn _broadcast(&self, psocks: Arc<Mutex<Vec<TcpStream>>>, msg: &str){
    let msg = msg.to_string();
    let handle = thread::spawn(move || {
      let mut handles = vec![];
      let socks = psocks.lock().unwrap();
      for mut sock in &(*socks) {
        let msg = msg.to_string();
        let handle = thread::spawn(move || {
          let mut rng = rand::thread_rng();
          let random_time: u64 = rng.gen();
          let sleep_time = time::Duration::from_millis(random_time);
          thread::sleep(sleep_time);
          sock.write(msg.as_bytes()).unwrap();
        });
        handles.push(handle);
      }
      for handle in handles {
        if let Err(e) = handle.join() {
          println!("Error broadcast");
        }
      }
    });

    handle.join();
  }

  fn random_sleep(&self) {
    let mut rng = rand::thread_rng();
    let random_time: u64 = rng.gen();
    let sleep_time = time::Duration::from_millis(random_time);
    thread::sleep(sleep_time);
  }
}
