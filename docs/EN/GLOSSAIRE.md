# Glossaire

### Kernel

This is the core of a computer operating system. It's the software that manages basic system operations, such as communication with hardware, process and memory management and program execution. The kernel provides an interface between the software and the hardware of the system, enabling this software to run efficiently and securely on the hardware.

The kernel is an essential component of any operating system and is typically the first program to be loaded into memory when the computer starts up. It's responsible for providing the basic services needed for all other programs to function properly. Some of the tasks performed by the kernel include:
- Memory management: The kernel allocates and manages system memory among the various processes and programs running on the computer.
- Process management: The kernel manages and coordinates the execution of processes in the system, allocating CPU time and resources to each process as needed.
- Communication with hardware: The kernel provides an interface for programs to access the system hardware, such as memory, input/output devices, and the network.
- Security: The kernel also handles ensuring the security of the system by restricting access to certain resources and providing mechanisms for protection against attacks and vulnerabilities.

### Boot

This is the process by which an operating system is loaded and executed on a computer.
It occurs in several stages, which are automatically executed when the computer is turned on, and requires the intervention of different software, such as the BIOS and the operating system's KERNEL.

### Linker

This is a tool used to link multiple binary files into a single executable file following the instructions of a linker script.
The linker reads this file and follows the contained instructions to know how to link the different binary files and where to place each part in the final executable file.

### What's a grub.cfg

It's a multiplatform bootloader used to boot the system. It defines the boot options and specifies how GRUB should behave during booting.
