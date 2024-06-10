# Grub.cfg

## Definition
Le fichier grub.cfg est un fichier de configuration utilisé par GRUB, un chargeur de démarrage multiplateforme, pour démarrer le système.

----

## Déroulement

`set timeout=15:` Fixe le délai d'attente avant le démarage automatiquement du système d'exploitation spécifié`  

`menuentry "KFS":` définis une entrée de menu dans le menu de démarrage de GRUB 

* `multiboot /boot/kernel:` Charge un fichier noyau de système d'exploitation en mémoire et le démarre

-----

## Exemple

Exemple grub avec plusieurs entrées.

```
menuentry "KFS" {
    multiboot /boot/kernel
}

menuentry "KFS(Safe Mode)" {
    multiboot /boot/kernel safe
}
```