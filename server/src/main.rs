#![feature(proc_macro_hygiene, decl_macro)]

mod auth;
mod database;
mod env;
mod server;
mod user_guard;

fn main() {
    server::init().unwrap().launch();
}
