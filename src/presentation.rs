use crate::media::MediaType;
use crate::media::Media;
use std::string::String;
use std::vec::Vec;

#[derive(Clone)]
pub struct Slide {
    pub text: String,
    pub background: Media,
}

impl Slide {
    pub fn blank_slide() -> Slide {
        Slide {
            text: "".to_owned(),
            background: Media {
                media_type: MediaType::Color,
                media: "#000000".to_owned(),
            },
        }
    }
}

#[derive(Clone)]
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

#[derive(Clone)]
pub struct Presentation {
    pub title: String,
    pub slides: Vec<Slide>,
    pub pres_type: PresType,
}

impl Presentation {
    pub fn blank_presentation() -> Presentation {
        let title = "None".to_owned();
        let slides: Vec<Slide> = Vec::new();
        let pres_type = PresType::Other("None".to_owned());

        Presentation {
            title,
            slides,
            pres_type,
        }
    }
}
