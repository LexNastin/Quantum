use crate::media::Media;
use std::string::String;
use std::vec::Vec;

pub struct Slide {
    pub text: String,
    pub background: Media,
}

pub enum PresType {
    Song,
    Scripture,
    Other(String),
}

impl PresType {
    pub fn as_string(&self) -> String {
        match self {
            PresType::Song => "Song".to_owned(),
            PresType::Scripture => "Scripture".to_owned(),
            PresType::Other(value) => value[..].to_owned(),
        }
    } 
}

pub struct Presentation {
    pub title: String,
    pub slides: Vec<Slide>,
    pub pres_type: PresType,
}
