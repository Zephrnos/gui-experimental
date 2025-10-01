// src/main.rs

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    // This calls the `run` function in your `lib.rs` file
    app_lib::run();
}