# Grub.cfg

## Definition


----

## Déroulement

`set timeout=15:`   

`menuentry "KFS":` 

* `multiboot /boot/kernel:` 

-----

## Exemple

Ejemplo de grub con varias entradas.

```
menuentry "KFS" {
    multiboot /boot/kernel
}

menuentry "KFS(Safe Mode)" {
    multiboot /boot/kernel safe
}
```