use libc::types::os::arch::c95::{c_int, size_t};
use std::ptr;

use usb::libusb::{
    libusb_device, libusb_exit, libusb_free_device_list, libusb_get_device_list, libusb_init,
};

fn main() {
    println!("Hello, world!");
    // unsafe calls to libusb
    unsafe {
        // use a null context (will use a default context)
        // let null_context: *mut *mut libusb_context = ptr::null_mut();
        let r: c_int = libusb_init(ptr::null_mut());
        if r < 0 {
            std::process::exit(r);
        }

        let mut devs: *mut *mut libusb_device = ptr::null_mut();
        let cnt: size_t = libusb_get_device_list(ptr::null_mut(), &mut devs);
        if cnt < 0 {
            libusb_exit(ptr::null_mut());
            std::process::exit(1);
        }
        println!("{:?}", devs);
        libusb_free_device_list(devs, 1);

        libusb_exit(ptr::null_mut());
    }
}
