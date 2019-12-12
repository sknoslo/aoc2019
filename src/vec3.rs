#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Vec3 {
    pub x: isize,
    pub y: isize,
    pub z: isize,
}

impl Vec3 {
    pub fn new(x: isize, y: isize, z: isize) -> Self {
        Self { x, y, z }
    }

    pub fn add(&mut self, other: &Self) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }

    pub fn mag(&self) -> isize {
        self.x.abs() + self.y.abs() + self.z.abs()
    }
}
