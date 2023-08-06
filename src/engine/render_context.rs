use crate::engine::scene::*;

pub struct RenderContext {
    pub scene: Scene,
    pub spp: u32,
    pub max_depth: u32,
    pub resolution: u32,
}

impl RenderContext {
    pub fn new() -> Self {
        Self {
            scene: Scene::new(),
            spp: 10,
            max_depth: 5,
            resolution: 1024
        }
    }
}