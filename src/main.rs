use autocxx::prelude::*;
include_cpp! {
    #include "repro.h"

    name!(repro)

    generate!("IBaseObject")
}

pub type Repro = repro::IBaseObject;

fn repro() -> Repro {
    todo!()
}

fn main() {
    let a = repro();
}
