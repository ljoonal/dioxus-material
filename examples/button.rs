use dioxus::prelude::*;
use dioxus_material::{use_theme_provider, Button, TextButton, Theme};

fn app(cx: Scope) -> Element {
    use_theme_provider(cx, Theme::default());

    render!(
        Button { onclick: |_| log::info!("clicked!"), "Click me!" }
        TextButton { onclick: |_| log::info!("clicked!"), "Click me!" }
    )
}

fn main() {
    dioxus_logger::init(log::LevelFilter::Info).expect("failed to init logger");

    dioxus_web::launch(app)
}
