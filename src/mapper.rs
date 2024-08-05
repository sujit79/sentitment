//! `Mapper` implementation for the sentiment project.
extern crate efflux;

use efflux::prelude::{Context, Mapper};

fn main() {
    // execute the mapping phase
    efflux::run_mapper(SentimentMapper);
}

/// The struct which will implement the `Mapper` trait.
struct SentimentMapper;

/// An empty implementation of the `Mapper` trait.
impl Mapper for SentimentMapper {
    fn setup(&mut self, _ctx: &mut Context) {
        // Carry out any setup required in this block.
    }

    fn map(&mut self, _key: usize, value: &[u8], ctx: &mut Context) {
        // Carry out the main mapping tasks inside this block.
        if value.is_empty() {
            return;
        }
        ctx.write(value, value);
    }

    fn cleanup(&mut self, _ctx: &mut Context) {
        // Carry out any cleanup required in this block.
    }
}
