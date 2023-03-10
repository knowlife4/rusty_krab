#[derive(Default)]
pub struct Vec2
{
    pub x: f32,
    pub y: f32,
}

impl Vec2
{
    pub fn new(x: f32, y: f32) -> Self
    {
        Vec2 { x, y }
    }
}

#[derive(Default)]
pub struct Vec3
{
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3
{
    pub fn new(x: f32, y: f32, z: f32) -> Self
    {
        Vec3 { x, y, z }
    }
}

#[derive(Default)]
pub struct Vec4
{
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl Vec4
{
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Self
    {
        Vec4 { x, y, z, w}
    }
}