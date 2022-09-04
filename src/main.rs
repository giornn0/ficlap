use routes::{AppRoutes, Section};
use sycamore::{prelude::*, suspense::Suspense};
use sycamore_router::{navigate, HistoryIntegration, Router};
use views::{
    festival::festival_view::Festival,
    index::{Index, IndexProps},
};
mod components;
mod models;
mod routes;
mod views;

const MAIN_TEXT: &str = "text-slate-300";

fn main() {
    console_log::init_with_level(log::Level::Debug).unwrap();
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    sycamore::render(|cx| {
        // create_effect(cx, || node_ref.get::<DomNode>().inner_element());
        view! {cx,
            Router(
                integration= HistoryIntegration::new(),
                view=|cx, route: &ReadSignal<AppRoutes>|{
                    let section_signal = create_signal(cx, Section::Home);
                    view!{cx,
                    main(class=format!("{}", MAIN_TEXT)){
                        (match route.get().as_ref(){
                            AppRoutes::Index{section}=> {
                                let index_props = IndexProps{document:document.clone(), section:section_signal};
                                view!{cx,
                                    Suspense(
                                        fallback= view!{cx,
                                            p(class="text-slate-300"){"Loading ..."}

                                    }){
                                        Index(index_props)
                                    }
                                }
                            },
                            AppRoutes::Festival=> view!{cx,Festival{}},
                            AppRoutes::NotFound=>{
                                section_signal.set(Section::Home);
                                navigate("/inicio");
                                view!{cx,}                            }
                        })}
                    }
                }
            )

        }
    });
}
