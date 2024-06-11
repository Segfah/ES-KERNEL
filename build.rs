// build.rs
fn main() {
    // Indica al compilador que utilice el script de enlace personalizado `linker.ld`.
    // El argumento `-T` especifica el script de enlace que el enlazador `ld` debe usar.
    println!("cargo:rustc-link-arg=-Tlinker.ld");

    // Configura el proyecto para recompilar automáticamente si `linker.ld` cambia.
    // Esto asegura que cualquier modificación en el script de enlace se maneje adecuadamente.
    println!("cargo:rerun-if-changed=linker.ld");

    // Establece el enlazador a utilizar como `ld`, permitiendo control directo sobre el proceso de enlace.
    // Usar `ld` directamente es común en el desarrollo de sistemas operativos para controlar con precisión cómo se combinan los módulos.
    //println!("cargo:rustc-linker-flavor=ld");
}
