# Kernel From Scratch

## What's a kernel?

## The computer booting sequence

* Power on
* Hardcoded jump to BIOS in ROM
* BIOS start the execution of the POST
* If test pass, the BIOS load MBR to RAM
* MBR loads bootloader (GRUB)
* Bootloader loads Operating System to RAM and leave execution to OS
The BIOS attempts to look for a bootable device.
After the button is pressed on the power supply, it provides electricity to main components like the CPU and the motherboard. 

## Resources

https://arjunsreedharan.org/post/82710718100/kernels-101-lets-write-a-kernel