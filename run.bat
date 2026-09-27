@echo off
chcp 65001 >nul
set HTTP_PROXY=http://127.0.0.1:10808
set HTTPS_PROXY=http://127.0.0.1:10808
cargo run --release
pause
