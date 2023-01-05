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
    pub fn to_string(&self) -> &str {
        match self {
            PresType::Song => "Song",
            PresType::Scripture => "Scripture",
            PresType::Other(value) => &value[..],
        }
    } 
}

pub struct Presentation {
    pub slides: Vec<Slide>,
    pub pres_type: PresType,
}
