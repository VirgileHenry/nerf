

pub struct TextAssets {
    font_system: cosmic_text::FontSystem,
    swash_cache: cosmic_text::SwashCache,
}


impl TextAssets {
    pub fn new() -> TextAssets {
        TextAssets {
            font_system: cosmic_text::FontSystem::new(),
            swash_cache: cosmic_text::SwashCache::new(),
        }
    }

    pub fn fonts_and_cache(&mut self) -> (&mut cosmic_text::FontSystem, &mut cosmic_text::SwashCache) {
        (&mut self.font_system, &mut self.swash_cache)
    }
}