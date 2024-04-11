pub fn ciclo_for_in() {
    // for trabaja con iterador

    let matriz = [1, 2, 3, 4, 5, 6, 7, 8, 9];

    for numero in matriz.iter() {
        println!("valor: {}", numero);
    }

    // Se puede definit un rango con ..
    // Si queremos que ultimo valo se inclya se utiliza el = ej 30..=40
    for numero in 30..40 {
        println!("valor en el rango: {}", numero);
    }
    // Existe la funcion rev(), que ejecuta el rango de forma inversa
    for numero in (35..=40).rev() {
        println!("valor en el rango: {}", numero);
    }
}

pub fn ciclo_while() {
    let matriz = [1, 2, 3, 4, 5, 6, 7, 8, 9];
    let mut i = 0;
    while i < 9 {
        println!("Posicion {} contiene el valor: {}", i, matriz[i]);
        i += 1;
    }
}

pub fn ciclo_loop() {
    // loop  -> Es un ciclo infinito, se tiene que trabajar con
    // break para detener el ciclo

    let mut contador = 0;

    let resultado = loop {
        println!("This is the iteracion {}", contador);
        contador += 1;
        if contador > 10 {
            break contador;
        }
    };

    println!("Iteracion final {}", contador);
}
