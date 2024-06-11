# Linker:

A linker script specifies the format and layout of the final executable binary. This script is needed to be recognized by the bootloader.

# Explanation of the script

The ENTRY keyword defines the entry point of an application. 

The OUTPUT_FORMAT keyword specifies the output format of the executable: a 32-bit ELF format for the i386 architecture.

The keyword SECTIONS

This section of the script defines how the different sections of the program are grouped and organized in the executable file. Each section can contain different types of data, such as code, initialized data and uninitialized data.

resource: https://wiki.osdev.org/Linker_Scripts