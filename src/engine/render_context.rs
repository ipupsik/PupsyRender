use crate::engine::render_target::{*};
use crate::engine::scene::{*};
use rand::rngs::ThreadRng;

pub struct RenderContext {
    pub render_target: RenderTarget,
    pub scene: Scene,
    pub spp: u64,
    pub max_depth: u64,
}