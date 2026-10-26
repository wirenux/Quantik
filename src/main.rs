#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // so no cmd open when building with --release

mod app;
mod camera;
mod math;
mod mesh;
mod molecule;
mod render;
mod scene;
mod ui;
mod window;

fn main() {
    app::run();
}
