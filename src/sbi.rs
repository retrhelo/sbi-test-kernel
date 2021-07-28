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
	fn fmt(&self, f: &mut fmt::Formatter<'_>) ->core::result::Result<(), fmt::Error> {
		write!(f, "{:?}", *self)
	}
}

// use core::result::Result;
type Result = core::result::Result<isize, SbiError>;

#[inline(always)]
fn sbi_call(eid: isize, fid: isize, arg0: isize, arg1: isize, arg2: isize) ->Result {
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

	if 0 == error {
		Ok(value)
	}
	else {
		Err(SbiError::from(error))
	}
}

#[inline(always)]
fn sbi_legacy_call(eid: isize, arg0: isize, arg1: isize, arg2: isize, arg3: isize) ->isize {
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
pub fn sbi_get_spec_version() ->Result {
	sbi_call(EID_BASE, 0, 0, 0, 0)
}

#[inline]
pub fn sbi_get_impl_id() ->Result {
	sbi_call(EID_BASE, 1, 0, 0, 0)
}

#[inline]
pub fn sbi_get_impl_version() ->Result {
	sbi_call(EID_BASE, 2, 0, 0, 0)
}

#[inline]
pub fn sbi_probe_extension(extension_id: isize) ->Result {
	sbi_call(EID_BASE, 3, 
			extension_id, 0, 0)
}

#[inline]
pub fn sbi_get_mvendorid() ->Result {
	sbi_call(EID_BASE, 4, 0, 0, 0)
}

#[inline]
pub fn sbi_get_marchid() ->Result {
	sbi_call(EID_BASE, 5, 0, 0, 0)
}

#[inline]
pub fn sbi_get_mimpid() ->Result {
	sbi_call(EID_BASE, 6, 0, 0, 0)
}

// Legacy Extensions 
// Well, according to RISC-V SBI spec, legacy functions generally 
// have no return values except sbi_console_getchar(). But to unify
// the SBI calling convention in our test-kernel, I suggest all SBI calls 
// returning Result. 

pub const EID_SET_TIMER: isize = 0;
pub const EID_CONSOLE_PUTCHAR: isize = 1;
pub const EID_CONSOLE_GETCHAR: isize = 2;
pub const EID_CLEAR_IPI: isize = 3;
pub const EID_SEND_IPI: isize = 4;
pub const EID_REMOTE_FENCE_I: isize = 5;
pub const EID_REMOTE_SFENCE_VMA: isize = 6;
pub const EID_REMOTE_SFENCE_VMA_ASID: isize = 7;
pub const EID_SHUTDOWN: isize = 8;

#[inline]
pub fn sbi_set_timer(stime_value: usize) ->Result {
	sbi_legacy_call(EID_SET_TIMER, stime_value as isize, 0, 0, 0);
	Ok(0)
}

#[inline]
pub fn sbi_console_putchar(ch: isize) ->Result {
	sbi_legacy_call(EID_CONSOLE_PUTCHAR, ch, 0, 0, 0);
	Ok(0)
}

#[inline]
pub fn sbi_console_getchar() ->Result {
	Ok(sbi_legacy_call(EID_CONSOLE_GETCHAR, 0, 0, 0, 0))
}

#[inline]
pub fn sbi_clear_ipi() ->Result {
	sbi_legacy_call(EID_CLEAR_IPI, 0, 0, 0, 0);
	Ok(0)
}

#[inline]
pub fn sbi_send_ipi(hart_mask: *const u32) ->Result {
	sbi_legacy_call(EID_SEND_IPI, hart_mask as isize, 0, 0, 0);
	Ok(0)
}

#[inline]
pub fn sbi_remote_fence_i(hart_mask: *const u32) ->Result {
	sbi_legacy_call(EID_REMOTE_FENCE_I, 
			hart_mask as isize, 0, 0, 0);
	Ok(0)
}

#[inline]
pub fn sbi_remote_sfence_vma(hart_mask: *const u32, start: u32, size: u32) ->Result {
	sbi_legacy_call(
		EID_REMOTE_SFENCE_VMA, 
		hart_mask as isize, 
		start as isize, 
		size as isize, 
		0
	);
	Ok(0)
}

#[inline]
pub fn sbi_remote_sfence_vma_asid(
	hart_mask: *const u32, 
	start: u32, size: u32, asid: u32
) ->Result {
	sbi_legacy_call(
		EID_REMOTE_SFENCE_VMA_ASID, 
		hart_mask as isize, 
		start as isize, 
		size as isize, 
		asid as isize
	);
	Ok(0)
}

// shutdown is very special. If implemented correctly, hart should never 
// return from this function
#[inline]
pub fn sbi_shutdown() ->Result {
	sbi_legacy_call(EID_SHUTDOWN, 0, 0, 0, 0);
	Err(SbiError::FAILED)
}