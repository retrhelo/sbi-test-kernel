// SBI Interface 

#![allow(unused)]

#[allow(non_camel_case_types)]
#[derive(Debug)]
pub enum SbiError {
	FAILED, 
	NOT_SUPPORTED, 
	INVALID_PARAM, 
	DENIED, 
	INVALID_ADDRESS, 
	ALREADY_AVAILABLE, 
	ALREADY_STARTED, 
	ALREADY_STOPPED, 
}

impl core::convert::From<isize> for SbiError {
	fn from(val: isize) ->Self {
		use SbiError::*;

		match val {
			-1 => FAILED, 
			-2 => NOT_SUPPORTED, 
			-3 => INVALID_PARAM, 
			-4 => DENIED, 
			-5 => INVALID_ADDRESS, 
			-6 => ALREADY_AVAILABLE, 
			-7 => ALREADY_STARTED, 
			_ => FAILED, 
		}
	}
}

use core::fmt;
impl fmt::Display for SbiError {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) ->Result<(), fmt::Error> {
		write!(f, "{:?}", *self)
	}
}

use core::result::Result;

#[derive(Debug)]
pub struct SbiRet {
	error: Result<(), SbiError>, 
	value: isize, 
}

#[inline(always)]
fn sbi_call(eid: isize, fid: isize, arg0: isize, arg1: isize, arg2: isize) ->SbiRet {
	let error: isize;
	let value: isize;

	unsafe {
		asm!(
			"ecall", 
			in("a7") eid, in("a6") fid, 
			in("a0") arg0, in("a1") arg1, in("a2") arg2, 
			lateout("a0") error, lateout("a1") value, 
		);
	}

	let error = if 0 == error {
		Ok(())
	}
	else {
		Err(SbiError::from(error))
	};

	SbiRet {
		error, value
	}
}

#[inline(always)]
fn sbi_legacy_call(eid: isize, arg0: isize) ->isize {
	let value: isize;

	unsafe {
		asm!(
			"ecall", 
			in("a7") eid, 
			in("a0") arg0, 
			lateout("a0") value, 
		);
	}

	value
}

// Base Extension
const EID_BASE: isize = 0x10;

#[inline]
pub fn sbi_get_spec_version() ->SbiRet {
	sbi_call(EID_BASE, 0, 0, 0, 0)
}

#[inline]
pub fn sbi_get_impl_id() ->SbiRet {
	sbi_call(EID_BASE, 1, 0, 0, 0)
}

#[inline]
pub fn sbi_get_impl_version() ->SbiRet {
	sbi_call(EID_BASE, 2, 0, 0, 0)
}

#[inline]
pub fn sbi_probe_extension(extension_id: isize) ->SbiRet {
	sbi_call(EID_BASE, 3, 
			extension_id, 0, 0)
}

#[inline]
pub fn sbi_get_mvendorid() ->SbiRet {
	sbi_call(EID_BASE, 4, 0, 0, 0)
}

#[inline]
pub fn sbi_get_marchid() ->SbiRet {
	sbi_call(EID_BASE, 5, 0, 0, 0)
}

#[inline]
pub fn sbi_get_mimpid() ->SbiRet {
	sbi_call(EID_BASE, 6, 0, 0, 0)
}

// Legacy Extensions 

#[inline]
pub fn sbi_set_timer(stime_value: usize) {
	sbi_legacy_call(0, stime_value as isize);
}

#[inline]
pub fn sbi_console_putchar(ch: isize) {
	sbi_legacy_call(1, ch);
}

#[inline]
pub fn sbi_console_getchar() ->u8 {
	sbi_legacy_call(2, 0) as u8
}

#[inline]
pub fn sbi_clear_ipi() {
	sbi_legacy_call(3, 0);
}

#[inline]
pub fn sbi_send_ipi(hart_mask: *const isize) {
	sbi_legacy_call(4, hart_mask as isize);
}

#[inline]
pub fn sbi_shutdown() {
	sbi_legacy_call(8, 0);
}