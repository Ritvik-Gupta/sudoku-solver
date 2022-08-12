#[derive(Clone, Copy, Hash, PartialEq, Eq, Debug)]
pub struct Vec2D(usize, usize);

impl std::ops::Add for Vec2D {
    type Output = Vec2D;

    fn add(self, other: Self) -> Self::Output {
        Vec2D(self.0 + other.0, self.1 + other.1)
    }
}

impl std::ops::Mul<usize> for Vec2D {
    type Output = Vec2D;

    fn mul(self, factor: usize) -> Self::Output {
        Vec2D(self.0 * factor, self.1 * factor)
    }
}

impl std::ops::Div<usize> for Vec2D {
    type Output = Vec2D;

    fn div(self, factor: usize) -> Self::Output {
        Vec2D(self.0 / factor, self.1 / factor)
    }
}

impl Vec2D {
    pub fn new(x: usize, y: usize) -> Self {
        Self(x, y)
    }

    pub fn x(&self) -> usize {
        self.0
    }

    pub fn x_mut(&mut self) -> &mut usize {
        &mut self.0
    }

    pub fn y(&self) -> usize {
        self.1
    }

    pub fn y_mut(&mut self) -> &mut usize {
        &mut self.1
    }

    pub fn project(&self, row_projection_factor: usize) -> usize {
        self.0 * row_projection_factor + self.1
    }
}
