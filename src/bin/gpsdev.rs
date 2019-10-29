extern crate udev;

// TODO: use `failure` crate for error interop
fn main() -> Result<(), udev::Error> {
    println!("Hello, world!");
    let context = udev::Context::new()?;
    let mut enumerator = udev::Enumerator::new(&context)?;

    enumerator.match_subsystem("tty")?;
    let devices = enumerator.scan_devices()?;

    for device in devices {
        println!("found device: {:?}", device.syspath());
        // println!("parent: {:?}", device.parent().unwrap().sysname());
        for property in device.properties() {
            println!("{:?} = {:?}", property.name(), property.value());
        }
        for attribute in device.attributes() {
            println!("{:?} = {:?}", attribute.name(), attribute.value());
        }
    }

    Ok(())
}
