fn main() {
    println!("cargo:rerun-if-changed=src/guarded_read.c");

    if std::env::var("CARGO_CFG_TARGET_OS").as_deref() != Ok("windows") {
        return;
    }

    cc::Build::new().file("src/guarded_read.c").compile("guarded_read");
}
