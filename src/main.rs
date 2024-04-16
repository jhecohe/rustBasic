mod cadenas;
mod ciclos;
mod enumeraciones;
mod estructuras;
mod estructuras2;
mod mapas;
mod metodos;
mod mi_match;
mod mi_match2;
mod propiedad;
mod referencias_prestamos;
mod segmentos;
mod vectores;

fn main() {
    // diferentes_ciclos();
    // propiedad();
    // ref_prestamos();
    // estructuras();
    // estructuras2();
    // metodos();
    // enumeraciones();
    // mi_match();
    // mi_match2();
    // vector();
    // cadenas();
    // mapas();
    segmentos();
}

fn segmentos() {
    segmentos::slice();
}

fn ref_prestamos() {
    referencias_prestamos::retornar_string();
}

fn diferentes_ciclos() {
    // ciclos::ciclo_loop();
    // ciclos::ciclo_while();
    // ciclos::ciclo_for_in();
}

fn propiedad() {
    // propiedad::propiedad_num();
    // propiedad::propiedad_cadena();
    propiedad::propiedad_cadena_clone();
}

fn estructuras() {
    estructuras::estructuras();
}

fn estructuras2() {
    estructuras2::ejecutar();
}

fn metodos() {
    metodos::ejecutar();
}

fn enumeraciones() {
    enumeraciones::ejecutar();
}

fn mi_match() {
    mi_match::ejecutar();
}

fn mi_match2() {
    mi_match2::ejecutar();
}

fn vector() {
    vectores::ejecutar();
}

fn cadenas() {
    cadenas::ejecutar();
}

fn mapas() {
    mapas::ejecutar();
}
