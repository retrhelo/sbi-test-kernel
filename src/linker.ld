OUTPUT_ARCH(riscv)
ENTRY(_entry)

/* 
	OpenSBI use 0x8020_0000 as the entry address, but in 
	PascSBI it's something different.
*/
BASE_ADDRESS = 0x80020000;

SECTIONS {
	. = BASE_ADDRESS;
	skernel = .;

	stext = .;
	.text : {
		*(.text.init)
		*(.text .text.*)
	}
	etext = .;

	srodata = .;
	.rodata : {
		*(.rodata .rodata.*)
		*(.srodata .srodata.*)
	}
	erodata = .;

	sdata = .;
	.data : {
		*(.data .data.*)
		*(.sdata .sdata.*)
	}
	edata = .;

	.bss : ALIGN(4K) {
		*(.bss.heap)
		*(.bss.stack)
		sbss = .;
		*(.bss .bss.*)
		*(.sbss .sbss.*)
	}
	ebss = .;

	ekernel = .;

	/DISCARD/ : {
		*(.eh_frame)
	}
}