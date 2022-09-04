use std::{
    fmt::{Display, Error},
    str::FromStr,
};
use sycamore_router::Route;

#[derive(Clone)]
pub enum Section {
    Home,
    Funciones,
    Informacion,
}
impl FromStr for Section {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "inicio" {
            return Ok(Section::Home);
        } else if s == "funciones" {
            return Ok(Section::Funciones);
        } else if s == "informacion" {
            return Ok(Section::Informacion);
        }
        Err(Error)
    }
}
impl Section {
    pub fn as_str<'a>(&self) -> &'a str {
        match self {
            Section::Home => "Home",
            Section::Funciones => "Funciones",
            Section::Informacion => "Informacion",
        }
    }
}

#[derive(Route)]
pub enum AppRoutes {
    #[to("/<section>")]
    Index { section: Section },
    #[to("/festival-actual")]
    Festival,
    #[not_found]
    NotFound,
}
impl Display for Section {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let displaying = match self {
            Section::Home => "Home",
            Section::Funciones => "Funciones",
            Section::Informacion => "Informacion",
        };
        write!(f, r"{displaying}")
    }
}
