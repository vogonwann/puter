// Generated by gir (https://github.com/gtk-rs/gir @ c954dbdc9ac0)
// from
// from gir-files (https://github.com/gtk-rs/gir-files.git @ 1dc2560a9ae8)
// DO NOT EDIT

#[cfg(not(docsrs))]
use std::process;

#[cfg(docsrs)]
fn main() {} // prevent linking libraries to avoid documentation failure

#[cfg(not(docsrs))]
fn main() {
    if let Err(s) = system_deps::Config::new().probe() {
        println!("cargo:warning={s}");
        process::exit(1);
    }
}
