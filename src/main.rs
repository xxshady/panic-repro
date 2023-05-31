use autocxx::prelude::*;
include_cpp! {
    #include "repro.h"

    name!(repro)

    generate!("Repro")
}

pub type Repro = repro::Repro;

fn repro() -> Repro {
    todo!()
}

fn main() {
    let _repro = repro();
}
