# Linker:

Un script de linkeur spécifie le format et la disposition du binaire exécutable final. Ce script doit être reconnu par le chargeur d'amorçage.

# Explication du script

Le mot-clé ENTRY définit le point d'entrée d'une application.

Le mot-clé OUTPUT_FORMAT spécifie le format de sortie de l'exécutable : un format ELF 32 bits pour l'architecture i386.

La mot-clé SECTIONS

Cette section du script définit comment les différentes sections du programme sont regroupées et organisées dans le fichier exécutable. Chaque section peut contenir différents types de données, telles que du code, des données initialisées et des données non initialisées.

Ressource : https://wiki.osdev.org/Linker_Scripts