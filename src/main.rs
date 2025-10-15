pub mod components;
pub mod service;

use chrono::Local;
use crate::components::menu_bar::MenuBar;
use crate::components::main_page::MainPage;
use gpui::{
    actions, prelude::*, App, Application
    , WindowOptions,
};
use crate::components::table::Table;

fn main() {
    Application::new().run(|cx: &mut App| {
        let table = Table::initialize();
        cx.activate(true);
        cx.on_action(quit);
        cx.on_action(log);
        cx.set_menus(MenuBar::default().menus);
        cx.open_window(WindowOptions::default(), |_, cx| cx.new(|_| MainPage {
            table
        }))
            .unwrap();
    });
}

actions!(set_menus, [Quit, Log]);

fn quit(_: &Quit, cx: &mut App) {
    println!("Gracefully quitting the application...");
    cx.quit();
}

fn log(_: &Log, _cx: &mut App) {
    println!("{}: Log...", Local::now());
}