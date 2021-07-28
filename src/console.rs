// Provides macros for general output

use core::fmt;

use crate::sbi::*;

struct Stdout;
impl Stdout {
	pub fn new() ->Self {
		Self
	}
}

impl fmt::Write for Stdout {
	fn write_str(&mut self, fmt: &str) ->fmt::Result {
		let mut buffer = [0u8; 4];
		for c in fmt.chars() {
			for code_point in c.encode_utf8(&mut buffer).as_bytes().iter() {
				sbi_console_putchar(*code_point as isize).unwrap();
			}
		}

		Ok(())
	}
}

use lazy_static::*;
use spin::Mutex;

lazy_static! {
	static ref CONSOLE_INST: Mutex<Option<Stdout>> = Mutex::new(None);
}

pub fn init() {
	*CONSOLE_INST.lock() = Some(Stdout::new());
}

pub fn _print(args: fmt::Arguments) {
	let mut console = CONSOLE_INST.lock();
	if console.is_none() {
		loop {}
	}
	else {
		use fmt::Write;
		console.as_mut().unwrap().write_fmt(args).unwrap();
	}
}

#[macro_export]
macro_rules! print {
	($fmt: literal $(, $($arg: tt)+)?) => {
		$crate::console::_print(format_args!($fmt $(, $($arg)+)?));
	}
}

#[macro_export]
macro_rules! println {
	($fmt: literal $(, $($arg: tt)+)?) => {
		$crate::console::_print(format_args!(concat!($fmt, "\n") $(, $($arg)+)?));
	}
}