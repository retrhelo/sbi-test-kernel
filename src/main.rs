#![no_std]
#![no_main]

#![feature(asm)]
#![feature(default_alloc_error_handler)]
#![feature(naked_functions)]
#![feature(fn_align)]

extern crate alloc;

#[macro_use]
mod console;	// provide println and print

mod heap;		// provide heap 

mod sbi;		// interface for SBI
mod base;		// tests for Base Extension
mod legacy;		// tests for Legacy Extensions

mod trap;		// trap handler

use core::panic::PanicInfo;
#[panic_handler]
#[allow(dead_code)]
fn panic(info: &PanicInfo) ->! {
	println!("\x33[31;1m[panic]\x33[0m: {}", info);
	loop {}
}

const STACK_SIZE: usize = 4 * 1024;
const STACK_OFFSET: usize = 12;
const NCPU: usize = 2;
const STACK_TOTAL_SIZE: usize = STACK_SIZE * NCPU;
#[link_section = ".bss.stack"]
static mut SBI_STACK: [u8; STACK_TOTAL_SIZE] = [0; STACK_TOTAL_SIZE];

#[naked]
#[no_mangle]
#[link_section = ".text.init"]
unsafe extern "C" fn _entry(_hartid: usize, _dtb: usize) ->! {
	// set up kernel stack and jump to rust_main
	asm!(r"
		mv t0, a0
		slli t0, t0, {offset}
		la sp, {stack_base}
		add sp, sp, t0

		j rust_main
	", 
		offset = const STACK_OFFSET, 
		stack_base = sym SBI_STACK, 
		options(noreturn), 
	)
}

#[no_mangle]
#[link_section = ".text"]
extern "C" fn rust_main(hartid: usize, _dtb: usize) ->! {		
	if 0 == hartid {
		// init heap and console
		heap::init();
		console::init();
		trap::init();

		let mask: u32 = 1 << 1;
		sbi::sbi_send_ipi(&mask).unwrap();
	}
	else {
		trap::init();
	}

	println!("hello hartid {}", hartid);

	if 0 == hartid {
		let mask: u32 = 1 << 1;
		sbi::sbi_send_ipi(&mask).unwrap();
	}
	legacy::run_tests(hartid);

	// endless loop 
	loop {
		unsafe {
			riscv::asm::wfi();
		}
	}

	unreachable!("unreachable codes!");
}