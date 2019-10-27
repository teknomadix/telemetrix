use std::thread;
use unbounded_gpsd::{types::*, GpsdConnection};

fn main() {
    println!("Hello, world!");

    thread::spawn(move || {
        // some work here
    });

    let mut gpsd_con = GpsdConnection::new("localhost:2947").expect("connection failed");
    gpsd_con.watch(true).expect("watch failed");
    loop {
        println!("getting res...");
        let res = gpsd_con.get_response().expect("get res failed");
        match res {
            Response::Tpv(tpv) => match tpv {
                TpvResponse::Fix3D { lat, lon, .. } => {
                    println!("{}, {}", lat, lon);
                }
                _ => {
                    println!("some other fix");
                }
            },
            _ => {
                println!("some other response");
            }
        }
    }
}
