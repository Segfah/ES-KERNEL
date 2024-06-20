/*
bits 32

MULTIBOOT_HEADER_MAGIC		equ 0x1BADB002
MULTIBOOT_HEADER_FLAGS		equ 0x0
MULTIBOOT_HEADER_CHECKSUM	equ -(MULTIBOOT_HEADER_MAGIC + MULTIBOOT_HEADER_FLAGS)

section .multiboot_header
	dd MULTIBOOT_HEADER_MAGIC
	dd MULTIBOOT_HEADER_FLAGS
	dd MULTIBOOT_HEADER_CHECKSUM

section .text
global start
extern main

start:
	mov esp, stack_space	;set stack pointer
	call main
	hlt			;halt the CPU

section .bss
resb 8192			;8KB for stack
stack_space:
*/

/// Multiboot header
#[repr(C)]
#[repr(align(4))]
pub struct MultibootHeader {
    pub magic: u32,
    pub flags: u32,
    pub checksum: u32,
}

pub const MULTIBOOT_HEADER: MultibootHeader = MultibootHeader {
    magic: 0x1BADB002,
    flags: 0,
    checksum: calculate_checksum(0x1BADB002, 0),
};

const fn calculate_checksum(magic: u32, flags: u32) -> u32 {
    (0u32.wrapping_sub(magic).wrapping_sub(flags)) as u32
}

// el encabezado multiboot esté en una sección específica
#[link_section = ".multiboot_header"]
#[no_mangle]
pub static MULTIBOOT: MultibootHeader = MULTIBOOT_HEADER;

