use std::path::PathBuf;

pub struct SpriteAsset {
    pub id: String,
    pub images: Vec<PathBuf>,
}

impl SpriteAsset {
    pub fn new(id: &str, images: Vec<&str>) -> Self {
        SpriteAsset {
            id: id.to_string(),
            images: images.iter().map(PathBuf::from).collect(),
        }
    }
}
