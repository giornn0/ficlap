use sycamore::prelude::*;

use super::components::{Events, Header, Sponsors};

pub fn Festival<G: Html>(cx: Scope) -> View<G> {
    view! {cx,
    Header{}
    Events{}
    Sponsors{}
    }
}
