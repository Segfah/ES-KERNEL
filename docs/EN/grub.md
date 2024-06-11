# Grub.cfg

## Definition
The grub.cfg file is a configuration file used by GRUB, a cross-platform bootloader, to start the system.

----

## Description

`set timeout=15:` Sets the waiting time before automatically booting the specified operating system

`set default=0:` Specifies the menu entry that will be selected by default. The index starts at 0.

`menuentry "KFS":` Defines a menu entry in the GRUB boot menu

* `multiboot /boot/kernel:` Loads an operating system kernel file into memory and starts it

-----

## Example

Example grub with multiple entries.

```
menuentry "KFS" {
    multiboot /boot/kernel
}

menuentry "KFS(Safe Mode)" {
    multiboot /boot/kernel safe
}
```
