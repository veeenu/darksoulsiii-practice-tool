check:
    cargo +nightly xwin clippy --target x86_64-pc-windows-msvc --all

test *args:
    cargo xwin test --target x86_64-pc-windows-msvc {{args}} -- --nocapture
