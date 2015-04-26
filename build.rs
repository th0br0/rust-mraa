use std::env;

fn build_i686() {
    match env::var_os("qecore_sdk_version") {
        Some(_) => println!("cargo:linker=i586-poky-linux-gcc"),
        _ => {}
    }
}

fn main () {

    match env::var("TRAVIS") {
        Ok(_) => println!("cargo:rustc-link-search=/tmp/install-prefix/lib"),
        _ => {}
    }

    match env::var("TARGET") {
        Ok(ref val) if val == "i686-unknown-linux-gnu" => build_i686(),
        _ => {}
    }

}
