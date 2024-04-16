// Vectores colocan todos los valores en posiciones de memoria consecutivas.
// Solo pueden almacenar valores del mismo tipo.

fn enumeraciones() {
    let fila = vec![
        CeldaHojaCalculo::Int(9),
        CeldaHojaCalculo::Float(9.60),
        CeldaHojaCalculo::String(String::from("Texto de ejemplo")),
    ];

    for value in &fila {
        println!("{:?}", value);
    }
    println!(
        "Valor de la ultima posicion -> {:?} ",
        &fila[fila.len() - 1]
    );
}

fn recorrer() {
    let mut v = vec![1, 2, 3, 4, 5];

    for i in &mut v {
        // El asterico se usa para poder modificar i. le dice al compilador que es mutable.
        *i += 10;
    }

    for i in &v {
        println!("{}", i);
    }
}

fn errores_apuntamiento() {
    let v = vec![1, 2, 3, 4, 5];

    // No tenemos la posicion 500, por lo que la funcion get NO RETORNA error
    // retorna un none
    let no_existe = v.get(500);
    println!("No contiene esa posicion (None): {:?}", no_existe);

    // Genera un error
    // "index out of bounds: the len is 5 but the index is 500"
    // let no_existe: &i32 = &v[500];

    let mut primero = &v[0];
    // Error -> No se puede modificar el vector mientras se usa una referencia.
    // v.push(6);
    // println!("El primer elemento es: {}", primero);
}

fn rust_interpreta() {
    let v = vec![1, 2, 3, 4, 5];

    let tercero: &i32 = &v[2];
    println!("El valor numero 3 = {}", tercero);

    match v.get(4) {
        Some(nmero) => println!("Concide con algun elemento"),
        None => println!("No concide con ningun elemento"),
    }
}

fn iniciar_vector() {
    let mut v: Vec<i32> = Vec::new();
    v.push(5);
    v.push(98);
    v.push(32);
    println!("Longitud de v = {}", v.len());

    v.pop();
    println!("Longitud de v = {}", v.len());
}

pub fn ejecutar() {
    // iniciar_vector();
    // rust_interpreta();
    // errores_apuntamiento();
    // recorrer();
    enumeraciones();
}

#[derive(Debug)]
enum CeldaHojaCalculo {
    Int(i32),
    Float(f32),
    String(String),
}
