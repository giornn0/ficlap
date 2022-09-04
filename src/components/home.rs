use sycamore::prelude::*;

use crate::Section;

pub fn Home<'a, G: Html>(cx: Scope<'a>, selected: &'a Signal<Section>) -> View<G> {
    view! {cx,
        main(class="grid grid-cols-2 gap-2 px-8 my-24 max-h-96"){
            img(src="assets/main.webp",class="w-full max-h-96"){
            }
            div(class="ml-16 h-full flex flex-col gap-4 justify-center content-between"){
                h2(class="text-center text-2xl text-red-500"){"28 de Septiembre al 01 de Octubre"}
                ul(class="list-disc text-xl text-stone-600"){
                    li{"Maestros internacionales"}
                    li{"Clases individuales y grupales"}
                    li{"Conversatorios"}
                    li{"Talleres"}
                    li{"Coro de Clarinetes del Festival con la paricipacion del Ensamble Sinfonico de San Martin de los Andes "}
                    li{"Orquesta Escuela Municipal de  Villa la Angostura"}
                }
                div(class="w-full flex justify-center"){
                    button(class="justify-self-center w-fit p-4 bg-teal-400 rounded-lg text-stone-600"){"Participar"}
                }
            }

        }
    }
}
