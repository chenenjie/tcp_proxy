#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]
extern crate futures;
extern crate tokio_core;
extern crate tokio_io;
#[macro_use]
extern crate error_chain;
extern crate toml;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate uuid;


use tokio_core::reactor::{Core, Handle};
use tokio_core::net::{TcpListener };
use tokio_core::net::TcpStream;
use tokio_io::{ AsyncRead};
use tokio_io::io;
use futures::{Stream};
use futures::future::Future;
use std::io::{ErrorKind, Read};
use std::thread;
use std::env;
use std::collections::HashMap;
use std::fs::File;

mod transfer;


pub mod proxy {
    error_chain!{
        types {
            Error, ErrorKind, ResultExt, Result;
        }

       foreign_links {
            Io(::std::io::Error);
            Toml(::toml::de::Error);
        }
    }

}

#[derive(Debug, Clone, Deserialize)]
struct Config{
    pub paths: HashMap<String, String>,
}

fn get_config() -> proxy::Result<Config>{
    let mut file_str = env::current_dir()?;
    file_str.push("proxy.toml");
    let mut file = File::open(file_str)?;
    let mut content = String::new();
    file.read_to_string(&mut content);

    let config: Config = toml::from_str(&content)?;

    Ok(config)
}


pub fn run(){
    if let Ok(config) = get_config() {

        let mut handles = vec![];
        for(key, value) in config.paths {
            handles.push(thread::spawn(move ||{
               serve(key.parse().unwrap(), value); 
            }));
        }

        for handle in handles {
            handle.join().unwrap();
        }
    }

}

pub fn serve(port: i32, proxy_addr: String) -> proxy::Result<()>{
    let mut core = Core::new().unwrap();
    let handle = core.handle();

    let addr = format!("0.0.0.0:{}", port).parse().unwrap();

    println!("bind {:?}", addr); 
    let listener = TcpListener::bind(&addr, &handle)?;

    let connections = listener.incoming();

    let server = connections.for_each(move |(socket, _peer_addr)| {
        let (writer, reader) = socket.split();

        let target_addr = proxy_addr.parse().unwrap();
        let peer_stream = TcpStream::connect(&target_addr, &handle);

        let deal = peer_stream.and_then(|stream| {
            let (peer_w, peer_r) = stream.split();
        
            let push = io::copy(writer, peer_r);
            let pull = io::copy(peer_w, reader);

            push.select(pull).then(|result|{
                match result {
                    Err((err, _)) => Err(ErrorKind::Other.into()),
                    Ok((item, _)) => Ok(item),
                }
            })
        }).then(|_| Ok(()));

        
        handle.spawn(deal);
        
        Ok(())
    });

    core.run(server).map_err(From::from)

}
