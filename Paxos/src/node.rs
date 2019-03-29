use std::io::prelude::*;
use std::io::{self, Read};
use std::net::TcpStream;
use std::thread;
use std::thread::JoinHandle;
use std::time;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::{Mutex, Arc};

use rand::prelude::*;

pub trait Node {

  fn get_connection_num(&self) -> usize;
  fn send(&self, idx: usize, msg: &str);
  fn broadcast(&self, msg: &str);

  fn recver(psocks: Arc<Mutex<Vec<TcpStream>>>) -> (Receiver<(usize, String)>, JoinHandle<()>) {
    let (tx, rx) = channel();
    let handle = thread::spawn(move || {
      loop {
        thread::sleep(time::Duration::from_millis(3));
        let mut num: usize = 0;
        let socks = psocks.lock().unwrap();
        for mut sock in &(*socks) {
          let mut data_len :[u8; 1] = [0];
          match sock.read_exact(&mut data_len) {
            Ok(_) => {
              let len = data_len[0] as usize;
              let mut content: Vec<u8> = vec![0; len];
              sock.read_exact(&mut content).expect("read contents fail");
              let msg = std::str::from_utf8(&content).unwrap();
              tx.send((num, msg.to_string()))
                .expect("interval node fail: can't transfer recving message in channels");
            },
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
              num += 1;
              continue;
            },
            Err(e) => panic!("encountered IO error: {}", e),
          };
          num += 1;
        }
      }
    });
    (rx, handle)
  }

  fn sender(psocks: Arc<Mutex<Vec<TcpStream>>>) -> (Sender<(usize, String)>, JoinHandle<()>) {
    let (tx, rx) : (Sender<(usize, String)>, Receiver<(usize, String)>) = channel();
    let handle = thread::spawn(move || {
      loop {
        let (idx, content) = rx.recv().unwrap();
        // random sleep
        let mut rng = rand::thread_rng();
        let random_time: u64 = rng.gen();
        let sleep_time = time::Duration::from_millis(random_time % 10 + 1);
        thread::sleep(sleep_time);
        // preprose the msg
        let mut content = content.as_bytes().to_owned();
        let mut msg: Vec<u8> = vec![content.len() as u8];
        msg.append(&mut content);
        // lock the stream and send
        let socks = psocks.lock().unwrap();
        let mut sock = &(*socks)[idx];
        sock.write(&msg)
          .expect("send fail");
      }
    });
    (tx, handle)
  }
}
