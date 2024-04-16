// Los metodos son similares a las funciones pero se definen
// dentro del contexto de una estructura (enumeracion, o objeto de rasgo)
// y su primer parametro es siempre self que representa la instancia
// de la estructura.

struct Rectangulo {
    ancho: u32,
    alto: u32,
}

impl Rectangulo {
    fn area(&self) -> u32 {
        self.ancho * self.alto
    }

    fn puede_contener(&self, otro: Rectangulo) -> bool {
        self.alto > otro.alto && self.ancho > otro.ancho
    }

    // Funciones Asociadas, son funciones que no toman como parametro a Slef
    // Se utilizan como constructores para devolver una instancia
    // ejemplo String::from
    fn cuadrado(lado: u32) -> Rectangulo {
        Rectangulo {
            alto: lado,
            ancho: lado,
        }
    }
}

pub fn ejecutar() {
    let rec1 = Rectangulo {
        ancho: 35,
        alto: 50,
    };

    let rec2 = Rectangulo {
        ancho: 35,
        alto: 50,
    };

    println!("Esta es el area de rec1: {}", rec1.area());
    println!("rec 1 contiene a rec2? : {}", rec1.puede_contener(rec2));
    let cuadrado = Rectangulo::cuadrado(35);
    println!("Alto del nuevo cuadrado: {}", cuadrado.alto);
}
