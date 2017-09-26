
use proxy;

use std::io::ErrorKind;
use tokio_core::net::TcpStream;
use tokio_core::reactor::Core;
use tokio_io::io::{copy, write_all, read_until};
use tokio_io::AsyncRead;
use futures::Future;

pub fn serve() -> proxy::Result<()> {
    let mut core = Core::new()?;
    let handle = core.handle();

    let addr = "0.0.0.0:12345".parse().unwrap();

    let touch = TcpStream::connect(&addr, &handle).and_then(|stream| {

        let confirm = write_all(stream, vec![1u8, 1]);        
        
        let uuid = "";
        let aim_addr = "127.0.0.1:10000".parse().unwrap();

        //let check_in = confirm.and_then(|(stream, _)| {
            //read_until(stream, '\n', Vec::new()) 
        //});
        
        let client_stream = TcpStream::connect(&addr, &handle);
        let bussiness_stream = TcpStream::connect(&aim_addr, &handle);

        let communication = client_stream.and_then(|client_stream|{
            bussiness_stream.and_then(|bussiness_stream|{
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
            })
        }).then(|_|Ok(())); 

        handle.spawn(communication);
        Ok(())
    });

    core.run(touch);
    Ok(())
}
