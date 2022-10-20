fn main() {
    println!("cargo:rustc-cdylib-link-arg=/DEF:lib/no-logo/exports.def");
}
