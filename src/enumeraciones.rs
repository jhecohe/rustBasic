//  Enumeraciones permiten definir un nuevo tipo de dato enumerando sus
// posibles variantes.
#[derive(Debug)]

enum tipo_ip {
    V4(String),
    V6(String),
}

// struct direccion_ip {
//     tipo: tipo_ip,
//     direccion: String,
// }

pub fn ejecutar() {
    let ip_v4 = tipo_ip::V4(String::from("127.0.0.1"));
    let ip_v6 = tipo_ip::V6(String::from("127.0.0.1::6"));
    println!("Esta es la ip: {:?}", ip_v4);
}
