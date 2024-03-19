use glam::{Vec2, Vec3};

pub trait IntoVec2 {
    fn into_vec2(&self) -> Vec2;
}

impl IntoVec2 for Vec3 {
    fn into_vec2(&self) -> Vec2 {
        Vec2::new(self.x, self.y)
    }
}

pub trait IntoVec3{
    fn into_vec3(&self) -> Vec3;
}

impl IntoVec3 for Vec2{
    fn into_vec3(&self) -> Vec3{
        Vec3::new(self.x, self.y, 0.)
    }
}

pub trait FromVec2<T>{
    fn from_vec_2(&self) -> T;
}

impl FromVec2<Vec3> for Vec3{
    fn from_vec_2(&self) -> Vec3{
        Vec3::new(self.x, self.y, 0.)
    }
}

