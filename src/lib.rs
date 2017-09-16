#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]
extern crate futures;
extern crate tokio_core;
extern crate tokio_io;
#[macro_use]
extern crate error_chain;

use tokio_core::reactor::{Core, Handle};
use tokio_core::net::{TcpListener };
use tokio_core::net::TcpStream;
use tokio_io::{ AsyncRead};
use tokio_io::io;
use futures::{Stream};
use futures::future::Future;
use std::io::{ErrorKind};


mod proxy {
    error_chain!{
        types {
            Error, ErrorKind, ResultExt, Result;
        }

       foreign_links {
            Io(::std::io::Error);
        }
    }

}


pub fn serve() -> proxy::Result<()>{
    let mut core = Core::new().unwrap();
    let handle = core.handle();

    let addr = "0.0.0.0:12345".parse().unwrap();
    let listener = TcpListener::bind(&addr, &handle)?;

    let connections = listener.incoming();

    let server = connections.for_each(move |(socket, _peer_addr)| {
        let (writer, reader) = socket.split();

        let target_addr = "127.0.0.1:10086".parse().unwrap();
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
