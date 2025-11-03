#![feature(popcorn_protocol)]
#![feature(asm_goto_with_outputs)]
#![feature(macro_metavar_expr_concat)]

extern crate alloc;

use std::os::popcorn::handle::OwnedHandle;
use std::path::PathBuf;
use proto::client::DeviceManager;
use proto::client::DeviceManagerTr;
use revolution_keyboard::{Action, KeyEvent};
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

	let layout = {
		let config = std::fs::read_to_string("fs:/config/revolution.toml")
				.expect("failed to read revolution config");
		let config = config.parse::<toml::Table>()
				.expect("failed to parse config");
		let layout = config.get("layout")
				.expect("no keyboard layout specified");
		let layout = layout.as_str()
				.expect("keyboard layout not a string");
		let layout = {
			let mut path = PathBuf::from("fs:/config/layouts");
			path.push(layout);
			path.set_extension("toml");
			path
		};
		std::fs::read_to_string(layout)
			.expect("failed to read keyboard layout")
	};
	let layout = toml::from_str::<revolution_keyboard::Layout>(&layout)
					.expect("failed to parse keyboard layout");
	println!("{:#?}", layout);

    let device_manager = OwnedHandle::<DeviceManager>::new("dev:", DeviceManager {})
		    .expect("failed to find device manager");

	for _ in 0..300 {
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

	let mut decoder = revolution_keyboard::State::new(layout);
	let mut s = String::new();

	loop {
		match keyboard.get_scancode() {
			Ok(scancode) => {
				match decoder.process_scancode(scancode as u16) {
					Some(KeyEvent::Text(text)) => s.push_str(&text),
					Some(KeyEvent::Action(Action::Enter)) => {
						println!("{s}");
						s.clear();
					}
					_ => {}
				}
			}
			Err(e) => println!("failed with error {e:?}"),
		}
	}
}
