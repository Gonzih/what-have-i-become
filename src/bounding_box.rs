use bevy::prelude::*;

pub struct BoundingBox {
    center: Vec3,
    size: Vec2,
}

impl BoundingBox {
    pub fn new(center: Vec3, size: Vec2) -> Self {
        Self { center, size }
    }

    pub fn point_in(&self, point: Vec2) -> bool {
        let start_x = self.center.x - self.size.x / 2.;
        let start_y = self.center.y - self.size.y / 2.;
        let end_x = self.center.x + self.size.x / 2.;
        let end_y = self.center.y + self.size.y / 2.;

        start_x < point.x && end_x > point.x && start_y < point.y && end_y > point.y
    }
}
