// Tests for Base Extension

#![allow(dead_code)]

use crate::sbi::*;

pub fn run_tests() {
	if sbi_probe_extension(0x10).is_ok() {
		test_get_spec_version();
		test_get_impl_id();
		test_get_impl_version();
		test_get_mvendorid();
		test_get_marchid();
		test_get_mimpid();
	}
	else {
		// Base Extension is required to be implemented in every SBI implementation, 
		// so panic if we find it's not implemented.
		panic!("Extension Base not implemented!");
	}
}

fn decode_version(value: isize) ->(isize, isize) {
	let major = (value >> 24) & 0x7f;
	let minor = value & 0xff_ffff;

	(major, minor)
}

fn test_get_spec_version() {
	let (major, minor): (isize, isize) = decode_version(
		sbi_get_spec_version().unwrap()
	);
	println!("Specification Ver: {}.{}", major, minor);
}

fn test_get_impl_id() {
	let id = sbi_get_impl_id().unwrap();
	println!("SBI Implementation: {:#x}", id as usize);
}

fn test_get_impl_version() {
	let (major, minor) = decode_version(
		sbi_get_impl_version().unwrap()
	);
	println!("SBI Implementation Version: {}.{}", major, minor);
}

fn test_get_mvendorid() {
	let mvendor = sbi_get_mvendorid().unwrap();
	println!("mvendorid: {:#x}", mvendor as usize);
}

fn test_get_marchid() {
	let marchid = sbi_get_marchid().unwrap();
	println!("marchid: {:#x}", marchid as usize);
}

fn test_get_mimpid() {
	let mimpid = sbi_get_mimpid().unwrap();
	println!("mimpid: {:#x}", mimpid as usize);
}