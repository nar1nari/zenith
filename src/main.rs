use app::App;
use leptos::prelude::*;

pub mod app;
pub mod astronomy;
pub mod bodies;
pub mod render;

fn main() {
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();

    mount_to_body(|| {
        view! {
            <App />
        }
    })
}
