use crate::models::{Location, Obra, Profesor, MONTHS};
use chrono::{prelude::*, NaiveDate, NaiveTime};
use sycamore::prelude::*;

#[derive(Prop, Clone)]
struct FunctionCardProps<'a> {
    profesors: Vec<Profesor>,
    date: NaiveDate,
    start: NaiveTime,
    description: &'a str,
    obra: Obra,
    location: Location,
    image: &'a str,
}

fn FunctionCard<'a, G: Html>(cx: Scope<'a>, props: FunctionCardProps<'a>) -> View<G> {
    view! {cx,
        div(class="w-1/2 mx-4 shadow-md bg-slate-100"){
            img(class="h-80 w-full",src=props.image){}
            div(class="relative -top-8 left-4 h-24 w-24 flex content-center flex-col w-fit text-white text-center rounded-full bg-red-500"){
                h2(class="text-2xl mt-4"){
                    (props.date.day())
                }
                h4(class="text-sm"){(MONTHS.get((props.date.month()-1) as usize).unwrap())}
            }
            div(class="text-center relative -top-16"){
                h3(class="text-black uppercase"){(props.obra)}
                p(class="text-stone-400"){(props.start.hour())"hs"}
                p(class="text-stone-400"){(props.location)}
            }
        }
    }
}

pub fn Functions<'a, G: Html>(cx: Scope<'a>) -> View<G> {
    let functions = vec![
        FunctionCardProps {
            profesors: vec![Profesor::new(
                "Nombre-Apellido",
                "Director kpito",
                "un genioo",
            )],
            date: NaiveDate::from_ymd(2022, 9, 28),
            start: NaiveTime::from_hms(21, 00, 00),
            description: "Testing the carsis",
            obra: Obra::ConciertoProfesores,
            location: Location::EscSMA,
            image: "assets/func1.webp",
        },
        FunctionCardProps {
            profesors: vec![Profesor::new(
                "Nombre-Apellido",
                "Director kpito",
                "un genioo",
            )],
            date: NaiveDate::from_ymd(2022, 9, 29),
            start: NaiveTime::from_hms(21, 00, 00),
            description: "Testing the carsis",
            obra: Obra::ConciertoProfesores,
            location: Location::EscSMA,
            image: "assets/func2.webp",
        },
    ];
    let view_functions = View::new_fragment(
        functions
            .iter()
            .map(|function| FunctionCard::<G>(cx, function.clone()))
            .collect(),
    );
    view! {cx,
        div(class="mx-4 my-8"){
            h2(class="uppercase ml-4 text-2xl text-red-500"){"Funciones"}
            ul(class="flex flex-row"){
                (view_functions)
            }
        }

    }
}
