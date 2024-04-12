// Es un tipo de dato personalizado que permite nombrar y
// empaquetar varios valores relacionados que forman un
// grupo con significado.
// Lo que otros lenguajes es un OBJETO que no contiene metodos.
// Las ESTRUCTURAS son similares a las TUPLAS, pueden contener
// diferentes tipos, la diferencia es que la estructura puede
// nombrar cada atributo.
// Por eso las estructuras son mas flexibles, ya que no dependen
// del orden de los datos para acceder a ellos.
// Cuando definmos un struct como mutable (mut), todos los valores
// son modificables.

struct Usuario {
    nombre: String,
    email: String,
    edad: i8,
    activo: bool,
}

fn nuevo_usuario(nombre: String, email: String, edad: i8) -> Usuario {
    // Cuando el parametro tiene el mismo nombre se puede dejar solo el valor
    // en ves de asignar nombre: nombre, -> nombre.
    Usuario {
        nombre,
        email,
        edad,
        activo: true,
    }
}

pub fn estructuras() {
    let usuario1 = nuevo_usuario(
        String::from("Jherson"),
        String::from("correo@correo.com"),
        40,
    );
    // Rust tiene la abilidad de completar la informacion de una estructura
    // en otra con los .. y la estructura que quiere copiar.
    let usuario2 = Usuario {
        nombre: String::from("Alfonso"),
        email: String::from("correo2@correo.com"),
        ..usuario1
    };

    println!("{} - {}", usuario1.nombre, usuario2.nombre);
    println!("{} - {}", usuario1.edad, usuario2.edad);
}
