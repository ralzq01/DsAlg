use std::io::prelude::*;
use std::net::TcpStream;


pub trait Node {
  fn send(&self, sock: &mut TcpStream, msg: &str) -> std::io::Result<()> {
    sock.write(msg)?;
    Ok(())
  }

  fn recv(&self, sock: &mut TcpStream) -> std::io::Result<String> {
    let mut msg = String::new();
    sock.read_to_string(&mut, &mut msg)?;
    Ok(msg)
  }

  fn broadcast(&self, socks: &mut Vec<TcpStream>, msg: &str) -> std::io::Result<()> {
    for sock in sock {
      sock.write(msg)?;
    }
    Ok(())
  }
}
