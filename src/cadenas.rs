// use std::fmt::format;

fn recorrer_cadena() {
    let cadena = String::from("Vamos a iterar la cadena");

    for caracter in cadena.chars() {
        print!("{}.", caracter);
    }
    println!("");
    for numerico in cadena.bytes() {
        print!("{}-", numerico);
    }
}

fn concatenar_cadenas() {
    let hola = String::from("Hola");
    let mundo = String::from("Mundo");
    // let hola_mundo = hola + &mundo;

    // recomendado para concatenar mas de 2 cadenas;
    let hola_mundof = format!("{}{}", hola, mundo);
    let mut hello = String::from("Hello");
    let friends = String::from("friendes");
    hello.push_str(&friends);
}

fn inicializar_cadenas() {
    let cad1 = String::from("Inicializar cadena");
    let cad2 = "Esto es una cadena".to_string();
    let mut cad3 = String::new();
    cad3.insert(0, 'c');
}

pub fn ejecutar() {
    recorrer_cadena();
}
