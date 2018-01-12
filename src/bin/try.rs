extern crate tcp_proxy;

use tcp_proxy::run;

fn main(){
    println!("try error {:?}", run());
}