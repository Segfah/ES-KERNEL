# Configuración básica
kfsVersion   ?= 0.0.1
isoName      ?= es-kernel-$(kfsVersion).iso
qemu         = qemu-system-i386
MKRESCUE     := grub-mkrescue

# Comandos comunes
RM           := /bin/rm -rf
MKDIR        := mkdir -p

# Directorios
dirIso       := iso
dirBoot      := $(dirIso)/boot
dirGrub      := $(dirBoot)/grub
dirSrc       := src

# Rutas de archivos
pathBinKfs   := target/i386/debug/es-kernel
linkerFile   := linker.ld
grubCfgFile  := grub.cfg

# Flags para ensamblador
asFlags      := -f elf32

# Colores para impresión en consola
CYAN         := \033[0;36m
GREEN        := \033[0;32m
RESET        := \033[m

# Reglas principales
.PHONY: all clean fclean re run build iso

all: build iso

build:
	@cargo build
	@printf "$(CYAN)Cargo build hecho$(RESET)\n"

iso:
	@$(MKDIR) $(dirGrub)
	@cp $(pathBinKfs) $(dirBoot)
	@cp $(grubCfgFile) $(dirGrub)
	@$(MKRESCUE) -o $(isoName) $(dirIso)
	@printf "$(GREEN)Iso creado$(RESET)\n"

clean:
	@$(RM) target Cargo.lock

fclean: clean
	@$(RM) $(isoName) $(dirIso)

re: fclean all

run: iso
	@$(qemu) -s -cdrom $(isoName)
