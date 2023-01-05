pub enum MediaType {
    Color,
    Video,
    Image,
    Audio,
}

pub struct Media {
    pub media_type: MediaType,
    pub media: String,
}
