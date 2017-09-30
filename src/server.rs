
use proxy;

use std::io::ErrorKind;
use tokio_core::net::TcpStream;
use tokio_core::reactor::Core;
use tokio_io::io::{copy, write_all};
use tokio_io::AsyncRead;
use futures::{Future, future, BoxFuture, Stream};
use bytes::BytesMut;
use std::io;
use tokio_io::codec::{Decoder, Encoder};
use tokio_service::{NewService, Service};
use std::str;


pub struct LineCodec;

impl Decoder for LineCodec {
    type Item = String;
    type Error = io::Error;

    fn decode(&mut self, buf: &mut BytesMut) -> io::Result<Option<String>> {
        if let Some(i) = buf.iter().position(|&b| b == b'\n') {
            // remove the serialized frame from the buffer.
            let line = buf.split_to(i);

            // Also remove the '\n'
            buf.split_to(1);

            // Turn this data into a UTF string and return it in a Frame.
            match str::from_utf8(&line) {
                Ok(s) => Ok(Some(s.to_string())),
                Err(_) => Err(io::Error::new(io::ErrorKind::Other,
                                             "invalid UTF-8")),
            }
        } else {
            Ok(None)
        }
    }
}


impl Encoder for LineCodec {
    type Item = String;
    type Error = io::Error;

    fn encode(&mut self, msg: String, buf: &mut BytesMut) -> io::Result<()> {
        buf.extend(msg.as_bytes());
        buf.extend(b"\n");
        Ok(())
    }
}

//struct EchoService;

//impl Service for EchoService{
    //type Request = String;
    //type Response = String;
    //type Error = io::Error;
    //type Future = BoxFuture<String, io::Error>;
    
    //fn call(&self, input: String) -> Self::Future{
        //future::ok(input).boxed()
    //}
//}

pub fn serve() -> proxy::Result<()>{ 
    let mut core = Core::new()?;
    let handle = core.handle();

    let addr = "0.0.0.0:12345".parse().unwrap();

    let touch = TcpStream::connect(&addr, &handle).and_then(|stream| {

        let confirm = write_all(stream, vec![1u8, 1]);

        let work = confirm.and_then(|(stream, _)| {
            let (writer, reader) = stream.framed(LineCodec).split();
            //let service = s.new_service()?;

            Ok((writer, reader))
        });


        let work = work.and_then(|(writer, reader)| {

            reader.for_each(|msg|{
                let v: Vec<&str> = msg.split('&').collect();

                if v.len() == 2 {
                    let uuid = v[0].to_owned();
                    let aim_addr = v[1].parse().unwrap();

                    let client_stream = TcpStream::connect(&target_addr, &handle1);
                    let bussiness_stream = TcpStream::connect(&aim_addr, &handle1);

                    let communication = client_stream.and_then(|client_stream|{
                        bussiness_stream.and_then(|bussiness_stream|{
                            Ok((client_stream, bussiness_stream))
                        })
                    });

                    let communication = communication.and_then(|(client_stream, bussiness_stream)| {
                        let (c_writer, c_reader) = client_stream.split();

                        let (b_writer, b_reader) = bussiness_stream.split();

                        let push = copy(c_writer, b_reader);
                        let pull = copy(b_writer, c_reader);

                        push.select(pull).then(|result|{
                            match result {
                                Err((err, _)) => Err(ErrorKind::Other.into()),
                                Ok((item, _)) => Ok(item),
                            }
                        })

                    });
                    let communication = communication.then(|_|Ok(())); 

                    handle.spawn(communication);
                }
                
                Ok(())
            })
        }).map_err(|_| ());
        handle.spawn(work);
        Ok(())
    });

    core.run(touch);
    Ok(())
}
