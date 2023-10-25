use std::{ops::{Add, Sub, Mul, Div}, f32::consts::PI};

#[derive(Clone, Copy)]
pub struct Radian {
    pub angle : f32,
}

impl Radian {
    pub const ZERO : Radian = Radian{ angle: 0. };
    pub const HALF : Radian = Radian{ angle: PI };
    pub const FULL : Radian = Radian{ angle: PI * 2. };
    
    pub fn normalize(self) -> Radian {
        let mut angle = self.angle;
        
        while angle > Radian::FULL.angle {
            angle -= Radian::FULL.angle;
        }

        while angle < 0. {
            angle += Radian::FULL.angle;
        }

        Radian { angle }
    }

    pub fn normalize_to_half(self) -> Radian {
        let mut angle = self.angle;

        
        while angle > Radian::HALF.angle {
            angle -= Radian::FULL.angle;
        }

        while angle < Radian::HALF.angle * -1. {
            angle += Radian::FULL.angle;
        }

        Radian { angle }
    }

    pub fn abs(self) -> Radian {
        Radian{ angle: self.angle.abs() }
    }
}

impl From<f32> for Radian {
    fn from(value: f32) -> Self {
        Radian { angle: value }
    }
}

impl Add for Radian {
    type Output = Radian;

    fn add(self, rhs: Self) -> Self::Output {
        Radian{ angle: self.angle + rhs.angle }.normalize()
    }
}

impl Sub for Radian {
    type Output = Radian;

    fn sub(self, rhs: Self) -> Self::Output {
        Radian{ angle: self.angle - rhs.angle}.normalize()
    }
}

impl Mul<f32> for Radian {
    type Output = Radian;

    fn mul(self, rhs: f32) -> Self::Output {
        Radian{ angle: self.angle * rhs}.normalize()
    }
}

impl Div<f32> for Radian {
    type Output = Radian;

    fn div(self, rhs: f32) -> Self::Output {
        Radian{ angle: self.angle / rhs}.normalize()
    }
}
