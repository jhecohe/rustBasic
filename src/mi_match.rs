fn valor_en_segundos(tiempo: &Tiempo) -> u32 {
    match tiempo {
        Tiempo::Segundo => {
            println!("Esto es un segundo");
            1
        }
        Tiempo::Minitu => 60,
        Tiempo::Hora => 3600,
        Tiempo::Dia(mes) => {
            println!("Este es el mes seleccionado {:?}", mes);
            86400
        }
    }
}

pub fn ejecutar() {
    let dia_enero = Tiempo::Dia(Mes::Enero);
    let t_segundos = valor_en_segundos(&dia_enero);
    println!(
        "La cantidad de segundo que tiene {:?} = {}",
        dia_enero, t_segundos
    );
}

#[derive(Debug)]
enum Mes {
    Enero,
    Febrero,
    Marzo,
    Abril,
    Mayo,
    Junio,
    Julio,
    Agosto,
    Septiembre,
    Octubre,
    Noviembre,
    Diciembre,
}

#[derive(Debug)]
enum Tiempo {
    Segundo,
    Minitu,
    Hora,
    Dia(Mes),
}
