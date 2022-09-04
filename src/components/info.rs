use sycamore::prelude::*;

pub fn Info<G: Html>(cx: Scope) -> View<G> {
    view! {cx,
        main(class="p-8 grid grid-cols-2 gap-4 my-24"){
            div(class=""){
                h1(class="uppercase text-2xl text-center text-red-500 mb-8"){"Fundamentos del Proyecto Artístico"}
                p(class="text-xl text-stone-600"){
                    "El ser humano en su esencia y característica de “ser
                    sociable”, busca
                    permanentemente comunicar. Una de las herramientas
                    más eficaces y completas
                    para ello es el ARTE que a través de sus diversas formas
                    permite la manifestación
                    de pensamientos, ideas, emociones y sentimientos,
                    influenciado por su relación y
                    visión del mundo. Desde la función práctica, el arte
                    satisface necesidades de
                    esparcimiento, distracción, encuentro con uno mismo,
                    búsqueda de la plenificación
                    personal psicológica y espiritual; e involucra también a
                    su entorno desde el
                    momento que activa el intelecto y los sentidos, despierta
                    el asombro o admiración
                    en quien expecta, al proyectarse en obras que tienen,
                    desde la creación personal,
                    características estéticas."
                }
            }
            img(src="assets/clarinete.jpg",class="w-full max-h-96"){
            }

        }
    }
}
