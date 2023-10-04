#![no_main]
#![no_std]

use log::info;
use uefi::{prelude::*, proto::console::text::Input, table::boot::SearchType, Identify};

#[entry]
fn main(_image_handle: Handle, mut system_table: SystemTable<Boot>) -> Status {
    uefi_services::init(&mut system_table).unwrap();
    let boot_services = system_table.boot_services();

    let input_handle = boot_services.get_handle_for_protocol::<Input>().unwrap();

    let mut input = boot_services
        .open_protocol_exclusive::<Input>(input_handle)
        .unwrap();

    loop {
        // SAFETY: This should be the only reference to the key event.
        let input_event = unsafe { input.wait_for_key_event().unsafe_clone() };
        boot_services
            .wait_for_event(&mut [input_event])
            .expect("Failed to wait for event");
        info!("{:#?}", input.read_key().unwrap());
    }
}
