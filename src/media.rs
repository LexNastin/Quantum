use std::string::String;

#[derive(Copy, Clone)]
pub enum MediaType {
    Color,
    Video,
    Image,
    Audio,
}

impl MediaType {
    pub fn as_string(&self) -> String {
        match self {
            MediaType::Color => "Color".to_owned(),
            MediaType::Video => "Video".to_owned(),
            MediaType::Image => "Image".to_owned(),
            MediaType::Audio => "Audio".to_owned(),
        }
    } 
}

#[derive(Clone)]
pub struct Media {
    pub media_type: MediaType,
    pub media: String,
}
