extern crate udev;

fn main() {
    println!("Hello, world!");
    let context = udev::Context::new().unwrap();
    let mut enumerator = udev::Enumerator::new(&context).unwrap();

    enumerator.match_subsystem("tty").unwrap();

    for device in enumerator.scan_devices().unwrap() {
        println!("found device: {:?}", device.syspath());
    }
}
