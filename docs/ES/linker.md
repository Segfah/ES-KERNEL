# Linker:

Un script de enlazador especifica el formato y diseño del binario ejecutable final. Este script debe ser reconocido por el cargador de arranque.

# Explicación del script

La palabra clave ENTRY define el punto de entrada de una aplicación.

La palabra clave OUTPUT_FORMAT especifica el formato de salida del ejecutable: un formato ELF de 32 bits para la arquitectura i386.

La palabra clave SECTIONS

Esta sección del script define cómo se agrupan y organizan las diferentes secciones del programa en el archivo ejecutable. Cada sección puede contener diferentes tipos de datos, como código, datos inicializados y datos no inicializados.

Recurso: https://wiki.osdev.org/Linker_Scripts