extern crate futures;
extern crate tokio_core;
extern crate futures_cpupool;

use futures::{Future, Stream};
use futures::Poll;
use futures::Async;
use tokio_core::reactor::Core;
use futures::executor::spawn;
use futures::task::current;
use futures_cpupool::CpuPool;
use futures::task::{current,Task};

struct Enjie{
    count: i32,
    task: Some(Task),
}

impl Enjie{
    fn new() -> Enjie {
        Enjie{
            count: 1,
            None,
        }
    }

    fn task(&mut self, task: Task) {
        self.task = task; 
    }
}

impl Stream for Enjie{
    
    type Item = String;
    type Error = ();

    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error>{
        if self.count > 10 {
            Ok(Async::Ready(None))
        }else if self.count < 5{
            self.count = self.count + 1;
            self.task(current());
            Ok(Async::NotReady)
        }else{
            self.count = self.count + 1;
            Ok(Async::Ready(Some(String::from("fuck")))) 
        }
    }
}

struct Foo{
    count: i32,
}

impl Foo{
    fn new() -> Foo {
        Foo{
            count: 1,
        }
    }
}

impl Future for Foo{
    type Item = String ;
    type Error = ();
    fn poll(&mut self) -> Poll<Self::Item, Self::Error>{
        if self.count == 1 {
            self.count = self.count + 1;
            println!("{:?}", self.count); 
            //current().notify();
            Ok(Async::NotReady)
        }else{
            Ok(Async::Ready(String::from("dick")))
        }
    }
}


fn main() {
    //let mut core = Core::new().unwrap();
    //let handle = core.handle();

    let cpupool = CpuPool::new(1); 


    let enjie = Enjie::new();

    let foo = Foo::new();
    let fuu = Foo::new();
    let fxx = Foo::new();

    let a = cpupool.spawn(foo);
    let b = cpupool.spawn(fuu);
    let k = cpupool.spawn(fxx);

    let c = a.wait().unwrap();

    println!("{}", c);


    //handle.spawn(foo.and_then(|text| {
        //println!("{}", text);
        //Ok(())
    //}));

    //let enjie = enjie.for_each(|text| {
        //println!("{}", text);

        
        //Ok(())
    //});
    //core.run(enjie);

}
