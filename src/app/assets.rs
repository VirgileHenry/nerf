


#[cfg(feature = "text")]
pub(crate) mod text_assets;




/// All assets that are used by the application.
pub struct Assets {
    #[cfg(feature = "text")]
    text: self::text_assets::TextAssets,
}

impl Assets {
    pub fn new() -> Assets {
        Assets {
            #[cfg(feature = "text")]
            text: self::text_assets::TextAssets::new()
        }
    }

    #[cfg(feature = "text")]
    pub fn text(&self) -> &self::text_assets::TextAssets {
        &self.text
    }

    #[cfg(feature = "text")]
    pub fn text_mut(&mut self) -> &mut self::text_assets::TextAssets {
        &mut self.text
    }
}