# Grub.cfg

## 

----

## 

`set timeout=15:`  

`menuentry "KFS":` 

* `multiboot /boot/kernel:` 

-----

## 

```
menuentry "KFS" {
    multiboot /boot/kernel
}

menuentry "KFS(Safe Mode)" {
    multiboot /boot/kernel safe
}
```