#![feature(popcorn_protocol)]
#![feature(asm_goto_with_outputs)]
#![feature(macro_metavar_expr_concat)]

extern crate alloc;

use std::os::popcorn::handle::OwnedHandle;
use std::path::PathBuf;
use proto::client::DeviceManager;
use proto::client::DeviceManagerTr;
use crate::keyboard_proto::client::HidKeyboard;
use crate::keyboard_proto::client::HidKeyboardTr;

#[macro_use]
#[path = "../../../driver/ps2/src/macros.rs"]
mod macros;

mod keyboard_proto {
	#![allow(unused)]

	pub mod client {
		use std::os::popcorn::handle::AsRawHandle;
		protocol! {
	        pub protocol HidKeyboard = 0x1003 {
	            ctor => {}

				fn get_scancode(&self) -> usize {
					unsafe {
						syscall!(1u128<<96 | UID, self.as_raw_handle().0 =>
							Ok(scancode) => {
								return Ok(scancode as usize);
							}
							Err(e) => {
								return Err(e);
							}
						);
					}
				}
	        }
	    }
	}
}

fn main() {
	println!("                                         ___
                                        |__ \\
  _ __   ___  _ __   ___ ___  _ __ _ __    ) |
 | '_ \\ / _ \\| '_ \\ / __/ _ \\| '__| '_ \\  / /
 | |_) | (_) | |_) | (_| (_) | |  | | | |/ /_
 | .__/ \\___/| .__/ \\___\\___/|_|  |_| |_|____|
 | |         | |    _
 |_|         |_|   |_) _     _  |    _|_ o  _ __
                   | \\(/_\\_/(_) | |_| |_ | (_)| |");

    let device_manager = OwnedHandle::<DeviceManager>::new("dev:", DeviceManager {})
		    .expect("failed to find device manager");

	for _ in 0..100 {
		std::thread::yield_now();
	}

	let mut count = 0;
	let keyboard = loop {
		std::thread::yield_now();
		println!("searching for keyboard...");
		let mut endpoints = device_manager.search_proto::<keyboard_proto::client::HidKeyboard>()
				.expect("failed to search for keyboards");
		if endpoints.len() > 0 { break endpoints.pop(); }
		else {
			count += 1;
			if count > 5 { panic!("failed to find keyboard") };
		}
	};
	println!("found keyboard at {keyboard:?}");

	let Some(keyboard) = keyboard else { panic!("failed to find keyboard") };

	let keyboard = PathBuf::from("dev:").join(keyboard);
	let keyboard = OwnedHandle::<HidKeyboard>::new(dbg!(keyboard), HidKeyboard {})
			.expect("unable to open keyboard");

	loop {
		match keyboard.get_scancode() {
			Ok(scancode) => println!("scancode {scancode:#x}"),
			Err(e) => println!("failed with error {e:?}"),
		}
	}
}
