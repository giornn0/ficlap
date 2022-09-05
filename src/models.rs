use std::fmt::Display;

#[derive(Clone)]
pub struct Profesor {
    name: String,
    avatar: String,
    title: String,
}
impl Profesor {
    pub fn new(name: &str, avatar: &str, title: &str) -> Self {
        Profesor {
            name: name.to_owned(),
            avatar: avatar.to_owned(),
            title: title.to_owned(),
        }
    }
}
#[derive(Clone)]
pub enum Obra {
    ConciertoProfesores,
    Martin,
    Culka,
}
impl Display for Obra {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let text = match self {
            Obra::ConciertoProfesores => "Concierto de Profesores",
            Obra::Martin => "Obra de Martin",
            Obra::Culka => "Obra de Culka",
        };
        write!(f, r"{text}")
    }
}
#[derive(Clone)]
pub enum Location {
    EscSMA,
}
impl Display for Location {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let text = match self {
            Location::EscSMA => "Escuela Superior de Música San Martin de los Andes",
        };
        write!(f, r"{text}")
    }
}

pub const MONTHS: [&str; 12] = [
    "Enero",
    "Febrero",
    "Marzo",
    "Abril",
    "Mayo",
    "Junio",
    "Julio",
    "Agosto",
    "Septiembre",
    "Octubre",
    "Noviembre",
    "Diciembre",
];
