use crate::engine::scene::*;
use rand::rngs::ThreadRng;

pub struct RenderContext {
    pub scene: Scene,
    pub spp: u32,
    pub max_depth: u32,
}

impl RenderContext {
    pub fn new() -> Self {
        Self {
            scene: Scene::new(),
            spp: 10,
            max_depth: 5
        }
    }
}