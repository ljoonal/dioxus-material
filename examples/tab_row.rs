use dioxus::prelude::*;
use dioxus_material::{TabRow, Tab};

fn app(cx: Scope) -> Element {
    render!(
        TabRow {
            tabs: cx
                .bump()
                .alloc([
                    render!(Tab { "Tab 1" }),
                    render!(Tab { "Tab 2" }),
                    render!(Tab { "Tab 3" }),
                ])
        }
    )
}

fn main() {
    dioxus_logger::init(log::LevelFilter::Info).expect("failed to init logger");

    dioxus_web::launch(app)
}
