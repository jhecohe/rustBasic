fn incrementar_uno(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

pub fn ejecutar() {
    let cinco = Some(5);
    let seis = incrementar_uno(cinco);

    print!("{:?}", seis);

    let dado = 5;
}
