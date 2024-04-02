use glam::{Vec2, Vec3};

pub struct RRectangle {
    pub half_size: Vec2,
    pub points: Vec<Vec3>,
}

impl RRectangle {
    pub fn new(half_size: Vec2) -> RRectangle {
        let points = vec![
            Vec3::new(-half_size.x / 2., -half_size.y / 2., 0.0),
            Vec3::new(-half_size.x / 2., half_size.y / 2., 0.0),
            Vec3::new(half_size.x / 2., half_size.y / 2., 0.0),
            Vec3::new(half_size.x / 2., -half_size.y / 2., 0.0),
            Vec3::new(-half_size.x / 2., -half_size.y / 2., 0.0),
        ];
        RRectangle { half_size, points }
    }
}



pub struct RCircle {
    pub radius: f32,
    pub points: Vec<Vec3>,
}

impl RCircle {
    pub fn new(radius: f32, num_points: usize) -> RCircle {
        let mut points = Vec::with_capacity(num_points);
        let angle_increment = std::f32::consts::PI * 2.0 / num_points as f32;

        for i in 0..=num_points {
            let angle = angle_increment * i as f32;
            let x = radius * angle.cos();
            let y = radius * angle.sin();
            points.push(Vec3::new(x, y, 0.0));
        }

        RCircle { radius, points }
    }
}