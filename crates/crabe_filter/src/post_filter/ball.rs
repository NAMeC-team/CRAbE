use nalgebra::Point2;
use crabe_framework::data::world::{Ball, World};
use crate::data::FilterData;
use crate::post_filter::PostFilter;

pub struct BallFilter;

impl PostFilter for BallFilter {
    fn step(&mut self, filter_data: &FilterData, world: &mut World) {
        world.ball = Some(filter_data.ball.data.clone());
    }
}