pub(crate) trait Vec3Like : Sized {
    fn x(&self) -> f64;
    fn y(&self) -> f64;
    fn z(&self) -> f64;

    fn from_xyz(x: f64, y: f64, z: f64) -> Self;
}

pub(crate) fn add<T: Vec3Like>(a: &T, b: &T) -> T {
    T::from_xyz(
        a.x() + b.x(),
        a.y() + b.y(),
        a.z() + b.z()
    )
}

pub(crate) fn sub<T: Vec3Like>(a: &T, b: &T) -> T {
    T::from_xyz(
        a.x() - b.x(),
        a.y() - b.y(),
        a.z() - b.z()
    )
}

pub(crate) fn neg<T: Vec3Like>(a: &T) -> T {
    T::from_xyz(
        -a.x(),
        -a.y(),
        -a.z()
    )
}

pub(crate) fn mul<T: Vec3Like>(a: &T, b: &T) -> T {
    T::from_xyz(
        a.x() * b.x(),
        a.y() * b.y(),
        a.z() * b.z()
    )
}

pub(crate) fn scalar_mul<T: Vec3Like>(a: &f64, b: &T) -> T {
    T::from_xyz(
        a * b.x(),
        a * b.y(),
        a * b.z()
    )
}

pub(crate) fn scalar_div<T: Vec3Like>(a: &f64, b: &T) -> T {
    T::from_xyz(
        b.x() / a,
        b.y() / a,
        b.z() / a
    )
}

pub(crate) fn display<T: Vec3Like>(vec: &T, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "({0}, {1}, {2})", vec.x(), vec.y(), vec.z())
}

macro_rules! impl_vec3_ops {
    ($t:ty) => {
        impl std::ops::Add for $t {
            type Output = $t;

            fn add(self, other: Self) -> Self::Output {
                $crate::vec3::add(&self, &other)
            }
        }

        impl std::ops::Add for &$t {
            type Output = $t;

            fn add(self, other: Self) -> Self::Output {
                $crate::vec3::add(&self, &other)
            }
        }

        impl std::ops::AddAssign for $t {
            fn add_assign(&mut self, other: Self) {
                *self = $crate::vec3::add(&self, &other);
            }
        }

        impl std::ops::Neg for $t {
            type Output = Self;

            fn neg(self) -> Self::Output {
                $crate::vec3::neg(&self)
            }
        }

        impl std::ops::Neg for &$t {
            type Output = $t;

            fn neg(self) -> Self::Output {
                $crate::vec3::neg(&self)
            }
        }

        impl std::ops::Sub for $t {
            type Output = $t;

            fn sub(self, other: Self) -> Self::Output {
                $crate::vec3::sub(&self, &other)
            }
        }

        impl std::ops::Sub for &$t {
            type Output = $t;

            fn sub(self, other: Self) -> Self::Output {
                $crate::vec3::sub(&self, &other)
            }
        }

        impl std::ops::SubAssign for $t {
            fn sub_assign(&mut self, other: Self) {
                *self = $crate::vec3::sub(&self, &other);
            }
        }

        impl std::ops::Mul for $t {
            type Output = $t;

            fn mul(self, other: Self) -> Self::Output {
                $crate::vec3::mul(&self, &other)
            }
        }

        impl std::ops::Mul for &$t {
            type Output = $t;

            fn mul(self, other: Self) -> Self::Output {
                $crate::vec3::mul(&self, &other)
            }
        }

        impl std::ops::MulAssign for $t {
            fn mul_assign(&mut self, other: Self) {
                *self = $crate::vec3::mul(&self, &other);
            }
        }

        impl std::ops::Mul<f64> for $t {
            type Output = $t;

            fn mul(self, other: f64) -> Self::Output {
                $crate::vec3::scalar_mul(&other, &self)
            }
        }

        impl std::ops::Mul<f64> for &$t {
            type Output = $t;

            fn mul(self, other: f64) -> Self::Output {
                $crate::vec3::scalar_mul(&other, &self)
            }
        }

        impl std::ops::MulAssign<f64> for $t {
            fn mul_assign(&mut self, other: f64) {
                *self = $crate::vec3::scalar_mul(&other, &self);
            }
        }

        impl std::ops::Div<f64> for $t {
            type Output = $t;

            fn div(self, other: f64) -> Self::Output {
                $crate::vec3::scalar_div(&other, &self)
            }
        }

        impl std::ops::Div<f64> for &$t {
            type Output = $t;

            fn div(self, other: f64) -> Self::Output {
                $crate::vec3::scalar_div(&other, &self)
            }
        }

        impl std::ops::DivAssign<f64> for $t {
            fn div_assign(&mut self, other: f64) {
                *self = $crate::vec3::scalar_div(&other, &self);
            }
        }

        impl std::fmt::Display for $t {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                $crate::vec3::display(self, f)
            }
        }


        impl $t where $t: Vec3Like {
            pub(crate) fn zero() -> Self {
                <$t>::from_xyz(0.0, 0.0, 0.0)
            }

            pub(crate) fn dot(&self, other: &Self) -> f64 {
                self.x() * other.x() + self.y() * other.y() + self.z() * other.z()
            }

            pub(crate) fn cross(&self, other: &Self) -> Self {
                Self::from_xyz(
                    self.y() * other.z() - self.z() * other.y(),
                    self.z() * other.x() - self.x() * other.z(),
                    self.x() * other.y() - self.y() - other.x()
                )
            }

            pub(crate) fn length_squared(&self) -> f64 {
                self.x()*self.x() + self.y()*self.y() + self.z()*self.z()
            }

            pub(crate) fn length(&self) -> f64 {
                self.length_squared().sqrt()
            }

            pub(crate) fn normalize(&self) -> Self {
                self/self.length()
            }
        }
    }
}

pub(crate) use impl_vec3_ops;


#[derive(Debug)]
pub(crate) struct Vec3 {
    x: f64, 
    y: f64, 
    z: f64
}

impl Vec3Like for Vec3 {
    fn x(&self) -> f64 {self.x}
    fn y(&self) -> f64 {self.y}
    fn z(&self) -> f64 {self.z}

    fn from_xyz(x: f64, y: f64, z: f64) -> Vec3 {Vec3 {x: x, y: y, z: z}}
}


impl_vec3_ops!(Vec3);