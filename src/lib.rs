#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]
extern crate futures;
extern crate tokio_core;
extern crate tokio_io;
#[macro_use]
extern crate error_chain;

use tokio_core::reactor::{Core, Handle};
use tokio_core::net::{Incoming, TcpListener };
use tokio_io::AsyncRead;
use futures::{Stream,Sink};


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


fn serve() -> proxy::Result<()>{
    let core = Core::new().unwrap();
    let handle = core.handle();

    let addr = "0.0.0.0:12345".parse().unwrap();
    let listener = TcpListener::bind(&addr, &handle)?;

    let connections = listener.incoming();

    let server = connections.for_each(move |(socket, _peer_addr)| {
        let (writer, reader) = socket.split();

        
        Ok(())
    });

    core.run(server)

}
