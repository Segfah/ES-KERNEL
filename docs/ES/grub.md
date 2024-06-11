# Grub.cfg

## Definición
El archivo grub.cfg es un archivo de configuración utilizado por GRUB, un gestor de arranque multiplataforma, para iniciar el sistema.

----

## Descripción

`set timeout=15:` Establece el tiempo de espera antes de iniciar automáticamente el sistema operativo especificado

`set default=0:` Especifica la entrada del menú que se seleccionará por defecto. El índice comienza en 0.

`menuentry "KFS":` define una entrada de menú en el menú de inicio de GRUB

* `multiboot /boot/kernel:` Carga un archivo de núcleo de sistema operativo en memoria y lo inicia

-----

## Ejemplo

Ejemplo de grub con varias entradas.

```
menuentry "KFS" {
    multiboot /boot/kernel
}

menuentry "KFS(Safe Mode)" {
    multiboot /boot/kernel safe
}
```
