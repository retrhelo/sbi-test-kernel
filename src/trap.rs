

#[naked]
#[no_mangle]
#[repr(align(4))]
unsafe extern "C" fn trap_vec() {
	// asm codes below are from xv6-k210's kerneltrap, 
	// which stores contexts on stack
	asm!("
		addi sp, sp, -256

		sd ra, 0(sp)
		sd gp, 16(sp)
		sd tp, 24(sp)
		sd t0, 32(sp)
		sd t1, 40(sp)
		sd t2, 48(sp)
		sd s0, 56(sp)
		sd s1, 64(sp)
		sd a0, 72(sp)
		sd a1, 80(sp)
		sd a2, 88(sp)
		sd a3, 96(sp)
		sd a4, 104(sp)
		sd a5, 112(sp)
		sd a6, 120(sp)
		sd a7, 128(sp)
		sd s2, 136(sp)
		sd s3, 144(sp)
		sd s4, 152(sp)
		sd s5, 160(sp)
		sd s6, 168(sp)
		sd s7, 176(sp)
		sd s8, 184(sp)
		sd s9, 192(sp)
		sd s10, 200(sp)
		sd s11, 208(sp)
		sd t3, 216(sp)
		sd t4, 224(sp)
		sd t5, 232(sp)
		sd t6, 240(sp)

		addi a0, sp, 256
		sd a0, 8(sp)

		mv a0, sp
		call trap_handler

		ld ra, 0(sp)
		ld gp, 16(sp)
		ld t0, 32(sp)
		ld t1, 40(sp)
		ld t2, 48(sp)
		ld s0, 56(sp)
		ld s1, 64(sp)
		ld a0, 72(sp)
		ld a1, 80(sp)
		ld a2, 88(sp)
		ld a3, 96(sp)
		ld a4, 104(sp)
		ld a5, 112(sp)
		ld a6, 120(sp)
		ld a7, 128(sp)
		ld s2, 136(sp)
		ld s3, 144(sp)
		ld s4, 152(sp)
		ld s5, 160(sp)
		ld s6, 168(sp)
		ld s7, 176(sp)
		ld s8, 184(sp)
		ld s9, 192(sp)
		ld s10, 200(sp)
		ld s11, 208(sp)
		ld t3, 216(sp)
		ld t4, 224(sp)
		ld t5, 232(sp)
		ld t6, 240(sp)

		addi sp, sp, 256

		sret", 
		options(noreturn), 
	);
}

#[derive(Debug)]
struct TrapFrame {
	ra: i64, 
	sp: i64, 
	gp: i64, 
	tp: i64, 
	t0: i64, 
	t1: i64, 
	t2: i64, 
	s0: i64, 
	s1: i64, 
	a0: i64, 
	a1: i64, 
	a2: i64, 
	a3: i64, 
	a4: i64, 
	a5: i64, 
	a6: i64, 
	a7: i64, 
	s2: i64, 
	s3: i64, 
	s4: i64, 
	s5: i64, 
	s6: i64, 
	s7: i64, 
	s8: i64, 
	s9: i64, 
	s10: i64, 
	s11: i64, 
	t3: i64, 
	t4: i64, 
	t5: i64, 
	t6: i64, 
}

use riscv::register::{
	scause, 
	scause::{Trap, Interrupt}, 
	sepc, stval, 
};

#[no_mangle]
extern "C" fn trap_handler(tf: &mut TrapFrame) {
	let cause = scause::read().cause();

	match cause {
		Trap::Interrupt(Interrupt::SupervisorTimer) => {
			println!("SupervisorTimer Interrupt");
			set_next_time();
		}, 
		Trap::Interrupt(Interrupt::SupervisorSoft) => {
			println!("SupervisorSoft Interrupt");
			super::sbi::sbi_clear_ipi().unwrap();
		}, 
		_ => {
			panic!(
				"Unhandled exception! scause: {:?}, sepc: {:016x?}, stval: {:016x?}, trap frame: {:p}, {:x?}",
				cause,
				sepc::read(),
				stval::read(),
				&tf as *const _,
				tf
			)
		}
	}
}

use riscv::register::{
	sstatus, stvec, sie, 
	stvec::TrapMode, 
};

pub fn init() {
	unsafe {
		stvec::write(trap_vec as usize, TrapMode::Direct);
		sie::set_ssoft();
		sie::set_stimer();
		sstatus::set_sie();
	}

	// set_next_time();
}

fn set_next_time() {
	let time: usize;
	unsafe {
		asm!("rdtime {0}", out(reg) time);
	}

	crate::sbi::sbi_set_timer(time + 10000000).unwrap();
}