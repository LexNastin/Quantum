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
            title: "Song Pres Test".to_owned(),
            slides: vec![
                Slide {text: "Song Slide 1".to_owned(), background: Media {media_type: MediaType::Color, media: "#000000".to_owned()}},
                Slide {text: "Song Slide 2".to_owned(), background: Media {media_type: MediaType::Color, media: "#000000".to_owned()}},
            ],
            pres_type: PresType::Song,
        };
        presentations.push(sample);
        let sample = Presentation {
            title: "Scripture Pres Test".to_owned(),
            slides: vec![
                Slide {text: "Scripture Slide 1".to_owned(), background: Media {media_type: MediaType::Image, media: "/media/image.png".to_owned()}},
                Slide {text: "Scripture Slide 2".to_owned(), background: Media {media_type: MediaType::Video, media: "/media/video.mp4".to_owned()}},
            ],
            pres_type: PresType::Scripture,
        };
        presentations.push(sample);
        let sample = Presentation {
            title: "Other Pres Test".to_owned(),
            slides: vec![
                Slide {text: "Other Test Slide 1".to_owned(), background: Media {media_type: MediaType::Audio, media: "/media/yes-this-is-possible.mp3".to_owned()}},
                Slide {text: "Other Test Slide 2".to_owned(), background: Media {media_type: MediaType::Audio, media: "/media/yes-this-is-possible.mp3".to_owned()}},
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

    pub fn as_string(&self) -> String {
        let mut presentation_strs = String::new();
        for item in self.presentations.iter() {
            let mut presentation_str = String::new();
            for slide in item.slides.iter() {
                presentation_str += "Background Type: ";
                presentation_str += &slide.background.media_type.as_string()[..];
                presentation_str += "\n";
                presentation_str += "Background Data: ";
                presentation_str += &slide.background.media[..];
                presentation_str += "\n";
                presentation_str += &slide.text[..];
                presentation_str += "\n";
            }
            presentation_strs += "Title: ";
            presentation_strs += &item.title[..];
            presentation_strs += "\n";
            presentation_strs += "Type: ";
            presentation_strs += &item.pres_type.as_string()[..];
            presentation_strs += "\n";
            presentation_strs += &presentation_str[..];
            presentation_strs += "\n";
        }
        presentation_strs[..presentation_strs.len() - 2].to_owned()
    }

    pub fn get_pres(&self, idx: usize) -> Presentation {
        self.presentations
            .get(idx)
            .unwrap_or(
                &Presentation::blank_presentation()
            )
            .clone()
    }
}
