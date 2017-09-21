
use proxy;

use tokio_io::io::read_exact;
use tokio_core::reactor::Core;
use tokio_core::net::TcpListener;
use futures::Stream;
use futures::future::Future;
use std::sync::mpsc::channel;
use uuid::Uuid;

pub fn trans() -> proxy::Result<()> {
    let mut core = Core::new().unwrap();
    
    let handle = core.handle();
    let addr = "0.0.0.0:56789".parse().unwrap();

    let listener = TcpListener::bind(&addr, &handle)?;

    //let (sender, receiver) = channel();
    

    let server = listener.incoming().for_each(move |(stream, peer_addr)| {
        let check = read_exact(stream, [0u8 ;2]).and_then(|(stream, buf)|{
            match buf[0]{
                0 => { 
                    println!("client stream");
                    let stream_id = Uuid::new_v4();

                }, 
                1 => {
                    match buf[1] {
                        0 => {
                            println!("server main stream");
                            loop {
                                //let signal = receiver.recv().unwrap();

                            }
                        },
                        1 => {
                            println!("server bussiness stream");
                        }, 
                        _ => {

                            println!("illegal stream")
                        },
                    }
                }, 
                _ => {
                    println!("illegal stream")
                }, 
            };
            Ok(())
        }).map_err(|_|());   

        handle.spawn(check);

        Ok(())

    });

    core.run(server).map_err(From::from)

}
