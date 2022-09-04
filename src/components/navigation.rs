use sycamore::prelude::*;

use crate::Section;

const ACTION_HOVER: &str = "hover:animate-pulse hover:bg-stone-300 hover:text-stone-700";

#[derive(Prop, Clone)]
struct ActionProp<'a> {
    #[builder(default)]
    bordered: bool,
    #[builder(default)]
    icon: &'a str,
    link: &'a str,
    section: Option<Section>,
    #[builder(default)]
    external: bool,
    #[builder(default)]
    text: &'a str,
    viewing: &'a Signal<Section>,
}
#[component]
fn Action<G: Html, 'a>(cx: Scope<'a>, props: ActionProp<'a>) -> View<G> {
    let section = props.section.clone();
    let handled_selection = move |_| {
        if let Some(section) = &section {
            props.viewing.set(section.clone());
        }
    };
    view! {cx,
        li(on:click=handled_selection,
           class=format!("{} {} {} mx-4 p-2 pt-3 w-fit cursor-pointer text-black",
                         if !props.icon.is_empty(){""}else{"rounded "},
                         if props.bordered {" border-2 border-stone-300"}else{""},ACTION_HOVER)){
            (if !props.link.len() >0 {
                view!{cx,
                    a(class="flex justify-center align-center" , href=format!("{}",props.link), target=if props.external{"_blank"}else{"_self"}){
                        (if !props.icon.is_empty(){
                            view!{cx, i(class=format!("{} fa-2x",props.icon))}
                        }else{
                            view!{cx,
                                span{
                                    (props.text.to_owned())
                                }
                            }
                        }
                    )}
                }
            }
            else{
                view!{cx,
                    span{
                        (props.text.to_owned())
                    }
                }
            })
        }
    }
}

#[component]
pub fn Navigation<G: Html, 'a>(cx: Scope<'a>, selected: &'a Signal<Section>) -> View<G> {
    let actions = vec![
        ActionProp {
            bordered: false,
            section: Some(Section::Home),
            icon: "",
            link: "",
            external: false,
            text: "HOME",
            viewing: selected,
        },
        ActionProp {
            bordered: false,
            section: Some(Section::Informacion),
            icon: "",
            link: "",
            external: false,
            text: "FESTIVAL",
            viewing: selected,
        },
        ActionProp {
            bordered: false,
            section: Some(Section::Funciones),
            icon: "",
            link: "",
            external: false,
            text: "CRONOGRAMA",
            viewing: selected,
        },
    ];
    let actions_views = View::new_fragment(
        actions
            .iter()
            .map(|action| Action::<G>(cx, action.clone()))
            .collect(),
    );
    view! {cx,
        header(class="h-24"){
            div(class="h-12 w-full bg-rose-300"){}
            div(class="grid grid-cols-3 h-fit bg-white gap-4 p-4"){
                div(class="h-12 flex "){
                    img(src="assets/logo1.jpg", class="object-contain h-12")
                    img(src="assets/logo2.jpg", class="ml-2 object-contain h-12")
                }
                div(class=" col-span-2"){
                    ul(class="actions flex flex-row align-end justify-around"){
                       (actions_views)
                    }
                }

            }
        }
    }
}
