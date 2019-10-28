extern crate udev;

fn main() {
    println!("Hello, world!");
    let context = udev::Context::new().unwrap();
    let mut enumerator = udev::Enumerator::new(&context).unwrap();

    enumerator.match_subsystem("tty").unwrap();

    for device in enumerator.scan_devices().unwrap() {
        println!("found device: {:?}", device.syspath());
        println!("parent: {:?}", device.parent().unwrap().sysname());
        for property in device.properties() {
            println!("{:?} = {:?}", property.name(), property.value());
        }
        for attribute in device.attributes() {
            println!("{:?} = {:?}", attribute.name(), attribute.value());
        }
    }
}
