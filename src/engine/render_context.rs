use crate::engine::scene::*;
use workerpool::Pool;
use workerpool::thunk::{Thunk, ThunkWorker};

extern crate num_cpus;

pub const TILE_SIZE: usize = 32;

pub struct RenderContext {
    pub scene: Scene,
    pub spp: u32,
    pub output: String,
    pub max_depth: u32,
    pub resolution: u32,
    pub debug_steps: bool,
}

impl RenderContext {
    pub fn new() -> Self {
        Self {
            scene: Scene::new(),
            spp: 100,
            output: String::from("test.png"),
            max_depth: 20,
            resolution: 1024,
            debug_steps: false,
        }
    }
}