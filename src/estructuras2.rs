// Area rectangulo
struct Rectangulo {
    ancho: u32,
    alto: u32,
}

pub fn ejecutar() {
    let rec = Rectangulo {
        ancho: 30,
        alto: 26,
    };

    let area1 = area(&rec);
    println!("Area: {}", area1)
}

fn area(rec: &Rectangulo) -> u32 {
    rec.ancho * rec.alto
}
