// Hashmap
use std::collections::{hash_map, HashMap};

fn numero_palabras_se_repiten() {
    let texto = String::from("Hola mundo maravilloso mundo");

    let mut map = HashMap::new();
    for palabra in texto.split_whitespace() {
        let contador = map.entry(palabra).or_insert(0);
        *contador += 1;
    }

    println!("{:?}", map);
}
fn si_contiene() {
    let mut puntaciones = HashMap::new();

    puntaciones.insert(String::from("warriors"), 121);
    puntaciones.insert(String::from("lakers"), 100);

    puntaciones.entry(String::from("sixers")).or_insert(130);
    puntaciones.entry(String::from("lakers")).or_insert(120);
    println!("{:?}", puntaciones);
}

fn modificar() {
    let mut puntaciones = HashMap::new();

    puntaciones.insert(String::from("warriors"), 121);
    puntaciones.insert(String::from("lakers"), 100);
    puntaciones.insert(String::from("sixers"), 130);

    puntaciones.insert(String::from("lakers"), 120);

    println!("-- Se actualiza la puntacion de los lakers");
    println!("{:?}", puntaciones);
}
fn recorrer() {
    let mut puntaciones = HashMap::new();

    puntaciones.insert(String::from("warriors"), 121);
    puntaciones.insert(String::from("lakers"), 100);
    puntaciones.insert(String::from("sixers"), 130);

    let equipo = String::from("Warriors");
    let puntuacion = puntaciones.get(&equipo);

    for (clave, valor) in &puntaciones {
        // El acceso a los valores es aleatorio.
        println!("Esta es la clave {} y el valor {}", clave, valor);
    }
}
fn def_con_vectores() {
    let teams = vec!["warriors", "lakers", "sixers"];
    let start_points = vec![121, 104, 130];

    let mut puntaciones: HashMap<_, _> = teams.into_iter().zip(start_points.into_iter()).collect();
}
fn definicion() {
    let mut puntaciones = HashMap::new();

    puntaciones.insert(String::from("warriors"), 121);
    puntaciones.insert(String::from("lakers"), 100);
    puntaciones.insert(String::from("sixers"), 130);
}
pub fn ejecutar() {
    // recorrer();
    // modificar();
    // si_contiene();
    numero_palabras_se_repiten();
}
