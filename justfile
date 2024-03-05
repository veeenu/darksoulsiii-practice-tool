set dotenv-load := true

check:
    cargo +nightly xwin clippy --target x86_64-pc-windows-msvc --all

test *args:
    cargo xwin test --target x86_64-pc-windows-msvc {{args}} -- --nocapture

install:
    cargo xtask dist
    cp jdsd_dsiii_practice_tool.toml \
      "${DSIII_TEST_PATCH_PATH}"
    cp target/x86_64-pc-windows-msvc/release/libjdsd_dsiii_practice_tool.dll \
      "${DSIII_TEST_PATCH_PATH}/dinput8.dll"
