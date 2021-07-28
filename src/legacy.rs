// Tests for Legacy Extensions

#![allow(dead_code)]

use crate::sbi::*;

pub fn run_tests() {
	if 0 != sbi_probe_extension(EID_SET_TIMER).unwrap() {
		test_set_timer();
	}
	if 0 != sbi_probe_extension(EID_CONSOLE_GETCHAR).unwrap() {
		test_console_getchar();
	}
}

fn test_set_timer() {}

fn test_console_getchar() {
	print!("text 10 chars here: ");
	for _ in 0..10 {
		let c = sbi_console_getchar().unwrap();
		sbi_console_putchar(c).unwrap();
	}
	println!("\n");
}

fn test_ipi() {}

// I have no idea how to test those fence SBI functions, but I'll list 
// them below, for future implementation. 

fn test_remote_fence_i() {}
fn test_remote_sfence_vma() {}
fn test_remote_sfence_vma_asid() {}

// We don't test shutdown here, because we'll call it at the end of kernel