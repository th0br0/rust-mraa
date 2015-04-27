use std::env;

fn build_i686() {
    if env::var("OECORE_SDK_VERSION").is_ok() {
        print!("cargo:linker=i586-poky-linux-gcc");
    }
}

fn main () {

    if env::var("TRAVIS").is_ok() {
        println!("cargo:rustc-link-search=/tmp/install-prefix/lib");
    }

    match env::var("TARGET") {
        Ok(ref val) if val == "i686-unknown-linux-gnu" => build_i686(),
        _ => {}
    }

}
