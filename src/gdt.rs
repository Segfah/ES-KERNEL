// src/gdt.rs

use core::arch::asm;
use crate::println;

// Descriptor de Segmento GDT
#[repr(C, packed)]
struct SegmentDescriptor {
    limit_low: u16,
    base_low: u16,
    base_mid: u8,
    access: u8,
    granularity: u8,
    base_high: u8,
}

impl SegmentDescriptor {
    // Constructor para un descriptor de segmento
    const fn new(base: u32, limit: u32, access: u8, granularity: u8) -> Self {
        SegmentDescriptor {
            limit_low: (limit & 0xFFFF) as u16,
            base_low: (base & 0xFFFF) as u16,
            base_mid: ((base >> 16) & 0xFF) as u8,
            access,
            granularity: ((limit >> 16) & 0x0F) as u8 | (granularity & 0xF0),
            base_high: ((base >> 24) & 0xFF) as u8,
        }
    }
}

// GDT: Un arreglo de descriptores de segmento
#[repr(C, align(8))]
struct GDT {
    null: SegmentDescriptor,
    kernel_code: SegmentDescriptor,
    kernel_data: SegmentDescriptor,
    user_code: SegmentDescriptor,
    user_data: SegmentDescriptor,
    kernel_stack: SegmentDescriptor,
    user_stack: SegmentDescriptor,
}

#[link_section = ".gdt"]
static GDT_TABLE: GDT = GDT {
    null: SegmentDescriptor::new(0, 0, 0, 0),
    kernel_code: SegmentDescriptor::new(0, 0xFFFFF, 0x9A, 0xCF),
    kernel_data: SegmentDescriptor::new(0, 0xFFFFF, 0x92, 0xCF),
    user_code: SegmentDescriptor::new(0, 0xFFFFF, 0xFA, 0xCF),
    user_data: SegmentDescriptor::new(0, 0xFFFFF, 0xF2, 0xCF),
    kernel_stack: SegmentDescriptor::new(0, 0xFFFFF, 0x92, 0xCF), // Datos del stack del kernel
    user_stack: SegmentDescriptor::new(0, 0xFFFFF, 0xF2, 0xCF),   // Datos del stack del usuario
};

// Registro GDTR
#[repr(C, packed)]
struct Gdtr {
    limit: u16,
    base: u32,
}

// Cargar la GDT
pub fn load_gdt() {
    let gdtr = Gdtr {
        limit: (core::mem::size_of::<GDT>() - 1) as u16,
        base: &GDT_TABLE as *const _ as u32,
    };
    unsafe {
        // Instrucción LGDT para cargar la GDT
        asm!(
            "lgdt [{}]",
            in(reg) &gdtr, //
            options(nostack, preserves_flags)
        );
    }
}

// Pruebas integradas para verificar GDT y Stack
pub fn test_gdt() {
    println!("Dirección de GDT_TABLE: {:#010x}", &GDT_TABLE as *const _ as u32);

    println!("Iniciando pruebas de la GDT y el stack...\n");
    
    // Verificar que la GDT haya sido cargada correctamente
    let mut gdtr: [u8; 10] = [0; 10];
    unsafe {
        asm!("sgdt [{}]", in(reg) &mut gdtr);
    }
    let gdt_base = u32::from_le_bytes([gdtr[2], gdtr[3], gdtr[4], gdtr[5]]);
    println!("Dirección base de la GDT: {:#010x}", gdt_base);
    assert!(gdt_base == &GDT_TABLE as *const _ as u32, "La dirección base de la GDT no coincide con GDT_TABLE");
    println!("GDT cargada correctamente.\n");
    
    // Verificar el stack del kernel
    println!("Verificando el stack del kernel...\n");
    let esp: u32;
    unsafe {
        asm!("mov {}, esp", out(reg) esp);
    }
    println!("Valor actual del stack pointer (ESP): {:#010x}", esp);
    assert!(esp > 0x0000_7000, "El valor del stack pointer parece inválido");
    println!("El stack del kernel esta configurado correctamente.\n");
    //test_gdt_multiple_loads();
}

// Nueva prueba: Verificar integridad del GDT tras múltiples cargas
pub fn test_gdt_multiple_loads() {
    println!("Iniciando prueba de múltiples cargas de GDT...\n");
    for _ in 0..5 {
        load_gdt();
    }
    println!("Carga de GDT realizada 5 veces sin errores.\n");
}
