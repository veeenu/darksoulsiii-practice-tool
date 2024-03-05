check:
    cargo xwin clippy --target x86_64-pc-windows-msvc

test *args:
    cargo xwin test --target x86_64-pc-windows-msvc {{args}} -- --nocapture
