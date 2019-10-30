extern crate udev;

// TODO: use `failure` crate for error interop
fn main() -> Result<(), udev::Error> {
    println!("Hello, world!");
    let context = udev::Context::new()?;
    let mut enumerator = udev::Enumerator::new(&context)?;

    enumerator.match_subsystem("tty")?;
    let devices = enumerator.scan_devices()?;

    let gps_devices = devices.filter(|device| {
        // u-blox AG, u-blox 7 [linux module: cdc_acm]
        device
            .properties()
            .find(|prop| prop.name() == "ID_VENDOR_ID" && prop.value() == "1546")
            .is_some()
            && device
                .properties()
                .find(|prop| prop.name() == "ID_MODEL_ID" && prop.value() == "01a7")
                .is_some()
    });

    for device in gps_devices {
        let dev_name = device
            .properties()
            .find(|prop| prop.name() == "DEVNAME")
            .map(|prop| prop.value().to_os_string());

        println!("GPS found device: {:?}", dev_name);
    }

    Ok(())
}
