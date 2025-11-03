use revolution_keyboard::KeyEvent;

fn main() {
	simple_logger::init_with_env();

	let mut args = std::env::args_os();
	let f = std::fs::read_to_string(args.nth(1).unwrap())
			.unwrap();
	let scancodes = std::fs::read_to_string(args.next().unwrap())
			.unwrap();
	let config = toml::from_str::<revolution_keyboard::Layout>(&f).unwrap();
	println!("{:#?}", config);

	let mut decoder = revolution_keyboard::State::new(config);
	let mut buf = String::new();

	for scancode in scancodes.split_whitespace() {
		let scancode = u16::from_str_radix(scancode, 16).unwrap();
		let key = decoder.process_scancode(scancode);
		if let Some(KeyEvent::Text(s)) = key {
			buf.push_str(&s);
		} else {
			println!("{:?}", key);
		}
	}

	println!();
	println!("{buf}");
}
