// SEGMENTO o SLICE Es un tipo de dato que no tiene propiedad y que permite hacer referencia a
// una secuencia contigua de elementos de una coleccion en lugar de a toda la
// coleccion

pub fn slice() {
    let cadena = String::from("Hola amigos del Rust");

    println!("La primera palabra es {}", get_first_word(&cadena))
}

fn get_first_word(cadena: &String) -> &str {
    let bytes = cadena.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &cadena[0..i]; // Si el segmento empieza desde la posicion 0 se puede obviar el 0
        }
    }
    &cadena[..] // Como se retorna toda la cadena se puede obviar el 0 y el ultimo valor
}
