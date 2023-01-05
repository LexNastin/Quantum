use crate::presentation::Presentation;
use crate::presentation::PresType;
use crate::presentation::Slide;
use crate::media::Media;
use crate::media::MediaType;
use std::vec::Vec;

pub struct Database {
    pub presentations: Vec<Presentation>,
}

impl Database {
    pub fn load_db() -> Database {
        let mut presentations: Vec<Presentation> = Vec::new();
        let sample = Presentation {
            slides: vec![
                Slide {text: "Song Slide 1".to_owned(), background: Media {media_type: MediaType::Color, media: "#000000".to_owned()}},
                Slide {text: "Song Slide 2".to_owned(), background: Media {media_type: MediaType::Color, media: "#000000".to_owned()}},
            ],
            pres_type: PresType::Song,
        };
        presentations.push(sample);
        let sample = Presentation {
            slides: vec![
                Slide {text: "Scripture Slide 1".to_owned(), background: Media {media_type: MediaType::Color, media: "#000000".to_owned()}},
                Slide {text: "Scripture Slide 2".to_owned(), background: Media {media_type: MediaType::Color, media: "#000000".to_owned()}},
            ],
            pres_type: PresType::Scripture,
        };
        presentations.push(sample);
        let sample = Presentation {
            slides: vec![
                Slide {text: "Other Test Slide 1".to_owned(), background: Media {media_type: MediaType::Color, media: "#000000".to_owned()}},
                Slide {text: "Other Test Slide 2".to_owned(), background: Media {media_type: MediaType::Color, media: "#000000".to_owned()}},
            ],
            pres_type: PresType::Other("Test".to_owned()),
        };
        presentations.push(sample);
        Database {
            presentations,
        }
    }

    pub fn add_presentation(&mut self, presentation: Presentation) {
        self.presentations.push(presentation)
    }
}
