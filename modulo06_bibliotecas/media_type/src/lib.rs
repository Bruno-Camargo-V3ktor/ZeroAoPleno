// Traits
pub trait Media {
    fn reproduce(&self) -> String;
}

// Structs
pub struct Movie {
    pub title: String,
    pub duration: u8,
}

pub struct Serie {
    pub title: String,
    pub seasons: u8,
}

// Impls
impl Movie {
    pub fn new(title: &str, duration: u8) -> Self {
        Self {
            title: title.to_string(),
            duration,
        }
    }
}

impl Media for Movie {
    fn reproduce(&self) -> String {
        let text = format!(
            "Playing the movie '{}' which is {} minutes long",
            &self.title, &self.duration
        );
        text
    }
}

impl Serie {
    pub fn new(title: &str, seasons: u8) -> Self {
        Self {
            title: title.to_string(),
            seasons,
        }
    }
}

impl Media for Serie {
    fn reproduce(&self) -> String {
        let text = format!(
            "Playing the series '{}' that has {} seasons",
            &self.title, &self.seasons
        );
        text
    }
}
