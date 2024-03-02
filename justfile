export CARGO_XTASK_DIST := "true"
cargo := if os_family() == "unix" { 
  "cargo xwin --target x86_64-pc-windows-msvc" 
} else { 
  "cargo" 
}

dist: xtask
  {{cargo}} xtask

xtask: 
  {{cargo}} build --release --package xtask
