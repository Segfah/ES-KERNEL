# Boot:

The boot process is the sequence of operations that a computer system performs when it is powered on or restarted.
The process initializes the hardware and loads the operating system.

### Boot process
The boot process involves several key steps:

1. Power-On Self Test (POST)
When the computer is powered on, the BIOS (Basic Input/Output System) or UEFI (Unified Extensible Firmware Interface) firmware performs a series of checks to ensure the hardware components (such as the CPU, memory, and peripheral devices) are functioning correctly.

If all checks pass, the system proceeds to the next step. If a critical error is detected, the system halts and may display an error message or beep code.

2. Bootloader Initialization
    * BIOS/UEFI: The BIOS/UEFI looks for a bootable device (e.g., hard drive, SSD, USB drive) and loads the bootloader from the Master Boot Record (MBR) or GUID Partition Table (GPT).
    * Bootloader: The bootloader is a small program responsible for loading the kernel into memory and starting its execution. Common bootloaders include GRUB, Syslinux, and systemd-boot.

3. Loading the Kernel
    * Kernel Image: The bootloader loads the kernel image into memory. For a Rust kernel, this image is typically an ELF (Executable and Linkable Format) file.
    * Memory Setup: The bootloader sets up the initial memory map, ensuring the kernel has access to the memory it needs.
    * Kernel Parameters: The bootloader may also pass parameters to the kernel, such as the location of the initramfs (initial RAM filesystem) and command-line arguments.

4. Transition to Protected Mode
This transition is essential to fully utilize the capabilities of modern x86 processors.
    * Protected Mode: Modern x86 processors start in real mode, which has a limited address space and capabilities. To access the full capabilities of the CPU, the system must switch to protected mode.
    * Long Mode: For 64-bit systems, the system also transitions to long mode, which enables 64-bit addressing and operations.
    * Steps:
        The bootloader enables the A20 line, typically by interacting with the keyboard controller or using the Fast A20 Gate method.
        Enable A20 Line: This step allows access to memory above 1 MB.
        Load GDT: The Global Descriptor Table (GDT) is loaded to define the memory segments.
        Switch to Protected Mode: The CPU is switched from real mode to protected mode by setting the appropriate bit in the control register.
        Switch to Long Mode: For 64-bit kernels, the system then switches to long mode.

5. Kernel Initialization
    * Entry Point: The CPU jumps to the kernel's entry point, a predefined memory address where the kernel's startup code begins executing.
    * Rust Runtime Initialization: The Rust runtime is initialized, which includes setting up the stack, initializing global variables, and preparing the heap (if needed).
    * Architecture-Specific Setup: Perform architecture-specific initialization tasks, such as configuring interrupt controllers, setting up paging (virtual memory), and initializing hardware devices.

6. Hardware Initialization
    * Device Drivers: The kernel initializes device drivers for various hardware components, such as the keyboard, display, storage devices, and network interfaces.
    * Interrupts: The interrupt descriptor table (IDT) is configured to handle hardware interrupts and exceptions.
    * Timers: System timers are set up to enable task scheduling and timekeeping.

7. System Services Initialization
    * Filesystem Mounting: The root filesystem is mounted, providing access to the rest of the operating system's files.
    * Process Scheduler: The process scheduler is initialized, allowing the kernel to manage and switch between multiple processes.
    * Initial Process: The kernel starts the initial process, often referred to as "init," which is responsible for starting other system services and user processes.

8. Handing Control to User Space
    * User Space Transition: The kernel transitions to user space, starting the first user-level process. This process is typically the init process, which then spawns other user-space services and applications.
    * Operational State: At this point, the system is fully operational, and the kernel continues to manage resources, handle interrupts, and provide services to user-space processes.

### Bootloader in Rust
The bootloader performs several key functions essential for initializing the environment required to run the Rust kernel.

The boot.rs file defines a Multiboot header, which is crucial for compatibility with Multiboot-compliant bootloaders like GRUB and QEMU. This header provides the bootloader with the information it needs to load the kernel.

* Multiboot Header: Contains the magic number (0x1BADB002), flags, and checksum required by the Multiboot specification.
* Checksum Calculation: Ensures the combined value of the magic number, flags, and checksum is zero, as required by the Multiboot specification.
* Placement: The #[link_section = ".multiboot_header"] attribute places the header in a specific section of the binary, where the bootloader expects to find it.
