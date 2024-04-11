// Stack vs Heap (Pilas vs Monton)
// Tanto steack como heap son parte de la memoria, para que el codigo
// las use, pero estas estructuradas de diferente manera

// The STACK o PILA almacena valores en el orden que los obtiene
// y elimina los valores en orden opuesto. LIFO y los valores
// deben tener un tamaño fijo conocido.
//
// The HEAP o MONTON alamcena datos con un tamaño desconocido en el momento
// de la compilacion o con un tamaño que podria cambiar.
// Cuando pones datos en el monton solicitas una cierta cantidad de memoria,
// el asignador encuentra un lugar vacio lo suficientemente grande  y lo marca
// como en uso y devuelve el puntero.
//
// REGLAS DE PROPIEDAD
// 1- Cada valor en Rust tiene una variable que se llama propietario.
// 2- Solo puede haber un propietario a la vez.
// 3- Cuando el propietario se sale del ambito, el valor se eliminara.
//  Ambito -> Lo que se encuentra dentro de un bloque de corchetes {}.

pub fn propiedad_cadena_error() {
    // Como las cadenas se almacenan en el HEAP, por que tienen tamaño
    // dinamico, al asignarse a otra variable se le asigna el puntero
    // a la otra variable, quedando la cadena1 sin puntero o se duplica
    // el puntero genrando un error. ahora la propiedad de cadena1
    // pertenece a cadeno2
    let cadena1 = String::from("Primera cadena");
    let cadena2 = cadena1;

    // println!("Cadena1 contiene: {}", cadena1);
    println!("Cadena2 contiene: {}", cadena2);
}

pub fn propiedad_cadena_clone() {
    // Para copiar un valor dinaico usamos la funcion clone()
    let cadena1 = String::from("Primera cadena");
    let cadena2 = cadena1.clone();

    println!("Cadena1 contiene: {}", cadena1);
    println!("Cadena2 contiene: {}", cadena2);
}

pub fn propiedad_cadena() {
    let cadena1 = String::from("Primera cadena");
    let cadena2 = cadena1;

    println!("Cadena2 contiene: {}", cadena2);
}

pub fn propiedad_num() {
    let i = 7;
    let j = i;

    println!("j: {}", j);
    println!("i: {}", i);
}
