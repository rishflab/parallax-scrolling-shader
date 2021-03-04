use std::path::PathBuf;

pub struct SpriteAsset {
    pub id: String,
    pub frames: Vec<PathBuf>,
}

impl SpriteAsset {
    pub fn new(id: &str, frames: Vec<&str>) -> Self {
        SpriteAsset {
            id: id.to_string(),
            frames: frames.iter().map(PathBuf::from).collect(),
        }
    }
}
