#[macro_use] extern crate failure;

use failure::Error;
use std::net::TcpListener;

#[derive(Debug, Fail)]
pub enum ProxyError {
    #[fail(display = "invalid proxy name: {}", name)]
    InvalidProxyName {
        name: String,
    },
    #[fail(display = "unknown proxy version: {}", version)]
    UnknownProxyVersion {
        version: String,
    }
}

pub fn run() -> Result<(), Error> {

    let listener = TcpListener::bind("0.0.0.0:23456")?;

    loop{
        let (stream, addr) = listener.accept()?;
    }
}


