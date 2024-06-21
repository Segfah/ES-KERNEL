/*
    Creating a Global Descriptor Table (GDT):

    1. Define the GDT structure with the necessary segments.
    2. Create a function to initialize and load the GDT.
    3. Ensure the GDT is placed at the specified memory address (0x00000800).
    4. Modify existing main.rs to initialize the GDT at startup.
    Include the necessary GDT entries for kernel code, data, stack,
    user code, data, and stack.
*/
// src/memory.rs

use crate::println;
use crate::vga::vga_buffer;
use core::arch::asm;

#[repr(C, packed)]
struct SegmentDescriptor {
    limit_low: u16,
    base_low: u16,
    base_middle: u8,
    access: u8,
    granularity: u8,
    base_high: u8,
}

impl SegmentDescriptor {
    const fn new(base: u32, limit: u32, access: u8, granularity: u8) -> SegmentDescriptor {
        SegmentDescriptor {
            limit_low: (limit & 0xFFFF) as u16,
            base_low: (base & 0xFFFF) as u16,
            base_middle: ((base >> 16) & 0xFF) as u8,
            access,
            granularity: ((limit >> 16) & 0x0F) as u8 | (granularity & 0xF0),
            base_high: ((base >> 24) & 0xFF) as u8,
        }
    }
}

#[repr(C, packed)]
struct GdtDescriptor {
    limit: u16,
    base: u32,
}

const GDT_SIZE: usize = 7;

// Address to place GDT
const GDT_ADDRESS: u32 = 0x00000800;

#[link_section = ".gdt"]
static mut GDT: [SegmentDescriptor; GDT_SIZE] = [
    SegmentDescriptor::new(0, 0, 0, 0),          // Null segment
    SegmentDescriptor::new(0, 0xFFFFF, 0x9A, 0xCF),  // Kernel Code segment
    SegmentDescriptor::new(0, 0xFFFFF, 0x92, 0xCF),  // Kernel Data segment
    SegmentDescriptor::new(0, 0xFFFFF, 0x92, 0xCF),  // Kernel Stack segment
    SegmentDescriptor::new(0, 0xFFFFF, 0xFA, 0xCF),  // User Code segment
    SegmentDescriptor::new(0, 0xFFFFF, 0xF2, 0xCF),  // User Data segment
    SegmentDescriptor::new(0, 0xFFFFF, 0xF2, 0xCF),  // User Stack segment
];

extern "C" {
    fn load_gdt(gdt: *const GdtDescriptor);
}

pub fn init_gdt() {
    let gdt_descriptor = GdtDescriptor {
        limit: (GDT_SIZE * core::mem::size_of::<SegmentDescriptor>() - 1) as u16,
        base: GDT_ADDRESS,
    };

    unsafe {
        // Copy the GDT to the desired address
        core::ptr::copy_nonoverlapping(&GDT as *const _ as *const u8, GDT_ADDRESS as *mut u8, core::mem::size_of_val(&GDT));
        // Call the assembly function to load the GDT
        load_gdt_asm(&gdt_descriptor);
    }
}

#[inline(always)]
unsafe fn load_gdt_asm(gdt_descriptor: &GdtDescriptor) {
    let gdt_ptr: *const GdtDescriptor = gdt_descriptor;
    // Get the address of the `limit` field
    let limit_ptr = core::ptr::addr_of!(gdt_descriptor.limit);
    asm!(
        "lgdt [{0}]", 
        "mov ax, 0x10", 
        "mov ds, ax", 
        "mov es, ax", 
        "mov fs, ax", 
        "mov gs, ax", 
        "mov ss, ax", 
        "ljmp $0x08, $next",
        // Pass the address of the `limit` field
        in(reg) gdt_ptr,
        next = sym next_fn_asm,
        options(noreturn)
    );
}

#[no_mangle]
extern "C" fn next_fn_asm() {
    unsafe {
        asm!("jmp $0, $next", in(reg) 0x08, sym next = _);
    }
}

extern "C" fn flush() {
    unsafe {
        asm!("ret");
    }
}

/*
    Printing the kernel stack in a human friendly way
*/
pub fn print_stack_trace() {
    let mut ebp: u32;
    unsafe {
        asm!("mov {}, ebp", out(reg) ebp);
    }

    println!("Stack trace:");
    while ebp != 0 {
        let eip: u32;
        unsafe {
            eip = *(ebp as *const u32).offset(1);
            ebp = *(ebp as *const u32);
        }
        println!("  {:08x}", eip);
    }
}
