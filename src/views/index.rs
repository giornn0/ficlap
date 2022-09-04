use crate::{
    components::{functions::Functions, home::Home, info::Info, navigation::Navigation},
    routes::Section,
};
use log::info;
use sycamore::prelude::*;
use web_sys::{Document, ScrollIntoViewOptions, ScrollLogicalPosition};

pub struct IndexProps<'a> {
    pub section: &'a Signal<Section>,
    pub document: Document,
}

#[component]
pub async fn Index<'a, G: Html>(cx: Scope<'a>, props: IndexProps<'a>) -> View<G> {
    let mut scroll_options = ScrollIntoViewOptions::new();
    scroll_options.block(ScrollLogicalPosition::End);
    create_effect(cx, move || {
        if let Some(test) = props
            .document
            .get_element_by_id(&props.section.get().to_string())
        {
            test.scroll_into_view_with_scroll_into_view_options(&scroll_options);
        }
    });
    view! {cx,
        Navigation(props.section)
        div(id=Section::Home ,class="home"){
            Home(props.section)
        }
        div(id=Section::Informacion ,class="informacion"){
            Info{}
        }
        div(id=Section::Funciones ,class="funciones"){
            Functions{}
        }

    }
}
