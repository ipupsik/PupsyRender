use crate::engine::render_target::{*};
use crate::engine::scene::{*};

pub struct RenderContext {
    pub render_target: RenderTarget,
    pub scene: Scene,
}