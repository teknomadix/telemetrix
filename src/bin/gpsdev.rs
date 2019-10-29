extern crate udev;

fn main() -> Result<(), udev::Error> {
    println!("Hello, world!");
    udev::Context::new()
        .and_then(|context| udev::Enumerator::new(&context))
        .and_then(|mut enumerator| {
            enumerator
                .match_subsystem("tty")
                .and(enumerator.scan_devices())
        })
        .and_then(|devices| {
            devices.for_each(|device| {
                println!("found device: {:?}", device.syspath());
                // println!("parent: {:?}", device.parent().unwrap().sysname());
                device.properties().for_each(|property| {
                    println!("{:?} = {:?}", property.name(), property.value());
                });
                device.attributes().for_each(|attribute| {
                    println!("{:?} = {:?}", attribute.name(), attribute.value());
                });
            });
            Ok(())
        })
}
