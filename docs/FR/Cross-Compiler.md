# Preparation du Cross-Compiler

### Créer le dossier où vous allez avoir votre code source
```
mkdir -pv ~/.bin
cd ~/.bin
```

### Preparation du prefix, target et path
```
export PREFIX="$HOME/.bin"
export TARGET=i386-elf
export PATH="$PREFIX/bin:$PATH"
```

### Téléchargement et decompression
```
wget https://github.com/westes/flex/releases/download/v2.6.4/flex-2.6.4.tar.gz
wget https://ftp.gnu.org/gnu/bison/bison-3.7.4.tar.gz
wget https://ftp.gnu.org/gnu/m4/m4-1.4.19.tar.gz
wget https://ftp.gnu.org/gnu/grub/grub-2.06.tar.xz
wget https://ftp.gnu.org/gnu/binutils/binutils-2.35.tar.gz
wget https://ftp.gnu.org/gnu/texinfo/texinfo-6.8.tar.gz

tar xvzf flex-2.6.4.tar.gz
tar xvzf bison-3.7.4.tar.gz
tar xvzf m4-1.4.19.tar.gz
tar -xvf grub-2.06.tar.xz
tar xvzf binutils-2.35.tar.gz
tar xvzf texinfo-6.8.tar.gz
```

### Installation de chaque paquet
```
cd binutils-2.35
./configure --target=$TARGET --prefix="$PREFIX" --with-sysroot --disable-nls --disable-werror
make
make install
cd ..
```
```
cd m4-1.4.19
./configure --target=$TARGET --prefix="$PREFIX" --disable-nls --enable-languages=c,c++ --without-headers
make
make install
cd ..
```
```
cd texinfo-6.8
./configure --target=$TARGET --prefix="$PREFIX" --disable-nls --enable-languages=c,c++ --without-headers
make
make install
cd ..
```
```
cd bison-3.7.4
./configure --target=$TARGET --prefix="$PREFIX" --disable-nls --enable-languages=c,c++ --without-headers
make
make install
cd ..
```
```
cd flex-2.6.4
./configure --target=$TARGET --prefix="$PREFIX" --disable-nls --enable-languages=c,c++ --without-headers
make
make install
cd ..
```
```
cd grub-2.06
./configure --target=$TARGET --prefix="$PREFIX" --with-sysroot --disable-nls --disable-werror
make
make install
cd ..
```

## Nettoyer le code source (optionnel)
```
rm -rf binutils-2.35 \
    m4-1.4.19 \
    texinfo-6.8 \
    bison-3.7.4 \
    flex-2.6.4 \
    grub-2.06
```

----
## Références
https://wiki.osdev.org/GCC_Cross-Compiler