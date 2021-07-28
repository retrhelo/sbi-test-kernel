#![no_std]
#![no_main]

#![feature(asm)]

extern crate alloc;

mod sbi;

// no stack, let's use SBI's stack directly
#[no_mangle]
fn _entry() ->! {
	

	loop {}
}