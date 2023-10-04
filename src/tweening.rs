use crate::prelude::*;

pub struct TransformProjectionLens {
    pub start: f32,
    pub end: f32,
}

impl Lens<OrthographicProjection> for TransformProjectionLens {
    fn lerp(&mut self, target: &mut OrthographicProjection, ratio: f32) {
        let value = self.start + (self.end - self.start) * ratio;
        target.scale = value;
    }
}
