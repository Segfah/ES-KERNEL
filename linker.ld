OUTPUT_FORMAT(elf32-i386)
ENTRY(_start)
SECTIONS
{
    . = 0x100000;

    .multiboot_header :
    {
        KEEP(*(.multiboot_header))
    }

    .text :
    {
        *(.text)
    }

    .data : { *(.data) }
    .bss  : { *(.bss)  }

    /* Adding a section for the Global Descriptor Table (GDT) */
    .gdt 0x00000800 : 
    {
        KEEP(*(.gdt))
    }
}
