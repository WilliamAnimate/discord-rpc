@echo off

cd src-tauri
cargo build --target x86_64-pc-windows-msvc --profile idfk -Z build-std=std,panic_abort -Z build-std-features=panic_immediate_abort