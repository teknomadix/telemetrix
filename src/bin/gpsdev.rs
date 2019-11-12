extern crate tokio;
extern crate udev;

use std::error::Error;
use tokio::fs;
use tokio::prelude::*;

// TODO: use `failure` crate for error interop
fn main() -> Result<(), Box<Error>> {
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

    // TODO warnings for failures getting device name?
    for device in gps_devices {
        device
            .properties()
            .find(|prop| prop.name() == "DEVNAME")
            .and_then(|prop| prop.value().to_os_string().into_string().ok())
            .map(|dev_name| {
                println!("GPS found device: {:?}", dev_name);
                let mut cmd = String::from("+");
                cmd.push_str(&dev_name[..]);
                println!("GPS command: {:?}", cmd);
                // TODO: Tokio write to unix socket
            });
    }

    Ok(())
}
