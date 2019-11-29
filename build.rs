#[cfg(not(target_os = "macos"))]
fn main() {
}

#[cfg(target_os = "macos")]
fn main() {
    cc::Build::new()
        .file("macos/osx.m")
        .compile("osx_shim")
    ;
}
