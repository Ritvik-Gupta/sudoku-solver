#[derive(Clone, Copy, Hash, PartialEq, Eq, Debug)]
pub struct Vec2D(pub usize, pub usize);

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
