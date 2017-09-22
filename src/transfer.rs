
use proxy;

use tokio_io::io::{copy, read_exact };
use tokio_core::reactor::Core;
use tokio_core::net::TcpListener;
use tokio_io::AsyncRead;
use futures::Stream;
use futures::future::Future;
use uuid::Uuid;
use std::collections::HashMap;
//use futures::sync::mpsc::unbounded;
use std::sync::mpsc::channel;
use std::io::{ErrorKind, Error};
use std::borrow::Borrow;
use std::cell::RefCell;
use std::rc::Rc;

pub fn trans() -> proxy::Result<()> {
    let mut core = Core::new().unwrap();
    
    let handle = core.handle();
    let addr = "0.0.0.0:56789".parse().unwrap();

    let listener = TcpListener::bind(&addr, &handle)?;

    //let (sender, receiver) = channel();

    let id_input_map = Rc::new(RefCell::new(HashMap::new()));
    let map_mirror = id_input_map.clone();

    let server = listener.incoming().for_each(move |(stream, peer_addr)| {
        let check = read_exact(stream, [0u8 ;2]);
        let map = map_mirror.clone();
        let handle1 = handle.clone();
        let deal = check.map(|(stream, buf)| (stream, buf, map)).and_then(move |(stream, buf, map)|{
            match buf[0]{
                0 => { 
                    println!("client stream");
                    let stream_id = Uuid::new_v4();
                    //wanted addr, port and id send to server main stream
                    //将内网地址 端口和 标识id发给server
                    let (tx, rx) = channel();
                    tx.send(stream);

                    map.borrow_mut().insert(format!("{}", stream_id), rx);
                }, 
                1 => {
                    match buf[1] {
                        0 => {
                            println!("server main stream");
                            loop {
                                //let signal = receiver.recv().unwrap();
                                //发完地址,端口和标识id就完成了

                            }
                        },
                        1 => {
                            println!("server bussiness stream");
                            //判断带来的标识id是否匹配 然后copy对应的stream

                            let stream_id = "";
                            let mut map_clone = map.borrow_mut();
                            let rx = map_clone.get_mut(stream_id.borrow()).unwrap();


                            let i_stream = rx.recv().unwrap();


                            let (in_writer, in_reader) = i_stream.split();
                            let (out_writer, out_reader) = stream.split();

                            let push = copy(in_writer, out_reader);
                            let pull = copy(out_writer, in_reader);


                            let communication = push.select(pull).then(|result|{
                                match result {
                                    Err((err, _)) => Err(()),
                                    Ok((item, _)) => Ok(item),
                                }
                            }).map(|_|()).map_err(|_|());

                            handle1.spawn(communication);
                        }, 
                        _ => {
                            println!("illegal stream");
                        },
                    }
                }, 
                _ => {
                    println!("illegal stream")
                }, 
            };
            Ok(())
        }).map_err(|_|());   

        handle.spawn(deal);

        Ok(())

    });
    core.run(server).map_err(From::from)
}


