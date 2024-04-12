mod ciclos;
mod estructuras;
mod propiedad;
mod referencias_prestamos;

fn main() {
    // diferentes_ciclos();
    // propiedad();
    // ref_prestamos();
    estructuras();
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
