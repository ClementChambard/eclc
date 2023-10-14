use crate::lexer::Location;

use std::ops::*;

#[derive(Debug, Clone)]
pub struct Located<T: Clone>(T, Location);

impl<T: Clone> Located<T> {
    pub fn new(t: T, loc: Location) -> Self {
        Self(t, loc)
    }

    pub fn loc(&self) -> &Location {
        &self.1
    }

    pub fn val(&self) -> &T {
        &self.0
    }

    pub fn into_val(self) -> T {
        self.0
    }
}

impl<T: Clone + Add<Output = T>> Add for Located<T> {
    type Output = Located<T>;

    fn add(self, other: Located<T>) -> Self::Output {
        Located(self.0 + other.0, self.1.merge(&other.1))
    }
}

impl<T: Clone + Sub<Output = T>> Sub for Located<T> {
    type Output = Located<T>;

    fn sub(self, other: Located<T>) -> Self::Output {
        Located(self.0 - other.0, self.1.merge(&other.1))
    }
}

impl<T: Clone + Mul<Output = T>> Mul for Located<T> {
    type Output = Located<T>;

    fn mul(self, other: Located<T>) -> Self::Output {
        Located(self.0 * other.0, self.1.merge(&other.1))
    }
}

impl<T: Clone + Div<Output = T>> Div for Located<T> {
    type Output = Located<T>;

    fn div(self, other: Located<T>) -> Self::Output {
        Located(self.0 / other.0, self.1.merge(&other.1))
    }
}

impl<T: Clone> From<T> for Located<T> {
    fn from(t: T) -> Self {
        Self(
            t,
            Location {
                line: 0,
                span: 0..1,
            },
        )
    }
}
