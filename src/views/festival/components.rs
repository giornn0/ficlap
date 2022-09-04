use sycamore::prelude::*;

pub fn Header<G: Html>(cx: Scope) -> View<G> {
    view! {cx,
        "Header Festival"
    }
}
pub fn Events<G: Html>(cx: Scope) -> View<G> {
    view! {cx,
        "Events"
    }
}
pub fn Sponsors<G: Html>(cx: Scope) -> View<G> {
    view! {cx,
    "Sponsors"
    }
}
pub fn Form<G: Html>(cx: Scope) -> View<G> {
    view! {cx,
        "Form"
    }
}
