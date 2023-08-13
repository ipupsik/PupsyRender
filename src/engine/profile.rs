use std::collections::HashMap;
use std::time::{Duration, Instant};

pub enum ProfileType {
    INCREMENTABLE,
    INSTANT
}

pub struct Profile {
    start: Instant,
    name: String,
    profile_type: ProfileType,
}

impl Profile {
    pub fn new(name: &str, profile_type: ProfileType) -> Self {
        Self {
            start: Instant::now(),
            name: String::from(name),
            profile_type: profile_type
        }
    }
}

impl Drop for Profile {
    fn drop(&mut self) {

        match self.profile_type {
            ProfileType::INSTANT => {
                println!("{}: {}", self.name, self.start.elapsed().as_millis());
            },
            _ => {},
        }
    }
}