pub fn retornar_string() {
    let s = String::from("Hola amigos");
    let str = String::from("Referencia de puntero");
    let mut prestamo = String::from("Te presto ");

    let (st, longitud) = calcula_longitud_valor(s);
    println!("{} tiene {} caracteres ", st, longitud);

    let lon = calcula_longitud_referencia(&str);
    println!("{} tiene {} caracteres ", str, lon);

    modificar_usando_puntero(&mut prestamo);
    println!("{}", prestamo);
}

fn calcula_longitud_valor(cadena: String) -> (String, usize) {
    let len = cadena.len();
    println!("{} tiene una longitud de {}:", cadena, len);

    (cadena, len)
}

fn calcula_longitud_referencia(cadena: &String) -> usize {
    // Cuando se usa el arpersan, se pasa el puntero en memoria
    // por lo cual la variable original no pierde la propiedad
    let lon = cadena.len();
    lon
}

fn modificar_usando_puntero(p: &mut String) {
    p.push_str("este monto");
}
