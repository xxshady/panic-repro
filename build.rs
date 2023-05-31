fn main() {
    let mut build = autocxx_build::Builder::new("src/main.rs", [&"src"])
        .extra_clang_args(&["-std=c++20"])
        .build()
        .unwrap();

    let flags = if cfg!(target_os = "windows") {
        ["/std:c++20"]
    } else if cfg!(target_os = "linux") {
        ["-std=c++2a"]
    } else {
        panic!("unsupported target_os");
    };

    for flag in flags {
        build.flag(flag);
    }

    build.compile("repro");
}
