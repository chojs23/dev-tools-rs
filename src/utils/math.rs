use std::ops::{Index, Mul, Neg};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Matrix1x3(pub [f32; 3]);

impl From<[f32; 3]> for Matrix1x3 {
    fn from(arr: [f32; 3]) -> Self {
        Self(arr)
    }
}

impl Index<usize> for Matrix1x3 {
    type Output = f32;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Matrix3(pub [[f32; 3]; 3]);

impl From<[[f32; 3]; 3]> for Matrix3 {
    fn from(arr: [[f32; 3]; 3]) -> Self {
        Self(arr)
    }
}

impl Matrix3 {
    #[allow(dead_code)]
    pub fn determinant(&self) -> f32 {
        self[0][0] * (self[1][1] * self[2][2] - self[1][2] * self[2][1])
            - self[0][1] * (self[1][0] * self[2][2] - self[1][2] * self[2][0])
            + self[0][2] * (self[1][0] * self[2][1] - self[1][1] * self[2][0])
    }

    #[allow(dead_code)]
    pub fn mul_by(&mut self, n: f32) {
        self.0[0][0] *= n;
        self.0[0][1] *= n;
        self.0[0][2] *= n;
        self.0[1][0] *= n;
        self.0[1][1] *= n;
        self.0[1][2] *= n;
        self.0[2][0] *= n;
        self.0[2][1] *= n;
        self.0[2][2] *= n;
    }

    #[allow(dead_code)]
    pub fn inverse(&self) -> Option<Matrix3> {
        let det = self.determinant();

        if det == 0. {
            return None;
        }

        let mut n = Matrix3::from([
            [
                self[1][1] * self[2][2] - self[1][2] * self[2][1],
                (self[0][1] * self[2][2] - self[0][2] * self[2][1]).neg(),
                self[0][1] * self[1][2] - self[0][2] * self[1][1],
            ],
            [
                (self[1][0] * self[2][2] - self[1][2] * self[2][0]).neg(),
                self[0][0] * self[2][2] - self[0][2] * self[2][0],
                (self[0][0] * self[1][2] - self[0][2] * self[1][0]).neg(),
            ],
            [
                self[1][0] * self[2][1] - self[2][0] * self[1][1],
                (self[0][0] * self[2][1] - self[0][1] * self[2][0]).neg(),
                self[0][0] * self[1][1] - self[0][1] * self[1][0],
            ],
        ]);

        n.mul_by(1. / det);

        Some(n)
    }
}

impl Mul<Matrix1x3> for Matrix3 {
    type Output = Matrix1x3;

    fn mul(self, rhs: Matrix1x3) -> Self::Output {
        let a = rhs[0] * self[0][0] + rhs[1] * self[0][1] + rhs[2] * self[0][2];
        let b = rhs[0] * self[1][0] + rhs[1] * self[1][1] + rhs[2] * self[1][2];
        let c = rhs[0] * self[2][0] + rhs[1] * self[2][1] + rhs[2] * self[2][2];
        [a, b, c].into()
    }
}

impl Index<usize> for Matrix3 {
    type Output = [f32; 3];

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl Mul for Matrix3 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        [
            [
                self[0][0] * rhs[0][0] + self[0][1] * rhs[1][0] + self[0][2] * rhs[2][0],
                self[0][0] * rhs[0][1] + self[0][1] * rhs[1][1] + self[0][2] * rhs[2][1],
                self[0][0] * rhs[0][2] + self[0][1] * rhs[1][2] + self[0][2] * rhs[2][2],
            ],
            [
                self[1][0] * rhs[0][0] + self[1][1] * rhs[1][0] + self[1][2] * rhs[2][0],
                self[1][0] * rhs[0][1] + self[1][1] * rhs[1][1] + self[1][2] * rhs[2][1],
                self[1][0] * rhs[0][2] + self[1][1] * rhs[1][2] + self[1][2] * rhs[2][2],
            ],
            [
                self[2][0] * rhs[0][0] + self[2][1] * rhs[1][0] + self[2][2] * rhs[2][0],
                self[2][0] * rhs[0][1] + self[2][1] * rhs[1][1] + self[2][2] * rhs[2][1],
                self[2][0] * rhs[0][2] + self[2][1] * rhs[1][2] + self[2][2] * rhs[2][2],
            ],
        ]
        .into()
    }
}

#[cfg(test)]
mod test_matrix {
    use crate::utils::math::{Matrix1x3, Matrix3};

    #[test]
    fn matrix3_determinant() {
        let got = Matrix3::from([[1., 2., 3.], [4., 5., 6.], [7., 2., 9.]]).determinant();
        let want = -36.0f32;

        assert!((got - want).abs() < f32::EPSILON);
    }

    #[test]
    fn matrix_inverse() {
        let got = Matrix3::from([[1., 2., 3.], [4., 5., 6.], [7., 2., 9.]])
            .inverse()
            .unwrap();
        let want = Matrix3::from([
            [-11. / 12., 1. / 3., 1. / 12.],
            [-1. / 6., 1. / 3., -1. / 6.],
            [3. / 4., -1. / 3., 1. / 12.],
        ]);
        assert_eq!(got, want);

        assert!(Matrix3::from([[1., 2., 3.], [4., 5., 6.], [7., 8., 9.]])
            .inverse()
            .is_none());
    }

    #[test]
    fn matrix3_mul_by_1x3() {
        let got = Matrix3::from([[1., 2., 3.], [4., 5., 6.], [7., 2., 9.]])
            * Matrix1x3::from([2., 3., 4.]);
        let want = Matrix1x3([20.0f32, 47., 56.]);
        assert_eq!(got, want);
    }

    #[test]
    fn matrix3_mul_by_3x3() {
        let a = Matrix3::from([[1., 2., 3.], [4., 5., 6.], [7., 2., 9.]]);
        let b = Matrix3::from([[7., 2., 9.], [4., 5., 6.], [1., 2., 3.]]);

        let got = a * b;
        let want = Matrix3::from([[18., 18., 30.], [54., 45., 84.], [66., 42., 102.]]);

        assert_eq!(got, want);
    }
}
#[inline(always)]
pub fn wrap_f32(num: f32) -> f32 {
    (num + 1.).fract()
}

#[inline(always)]
pub fn eq_f32(lhs: f32, rhs: f32) -> bool {
    (lhs - rhs).abs() < f32::EPSILON
}

#[cfg(test)]
mod tests {
    use super::{eq_f32, wrap_f32};

    #[test]
    fn it_wraps() {
        assert!(eq_f32(wrap_f32(0.0f32), 0.0f32));
        assert!(eq_f32(wrap_f32(-1. / 8.), 7. / 8.));
        assert!(eq_f32(wrap_f32(-1. / 2.), 1. / 2.));
        assert!(eq_f32(wrap_f32(-1.), 0.));
        assert!(eq_f32(wrap_f32(1. / 8.), 1. / 8.));
        assert!(eq_f32(wrap_f32(1. / 2.), 1. / 2.));
        assert!(eq_f32(wrap_f32(1.), 0.));
    }

    #[test]
    fn it_eqs() {
        assert!(eq_f32(0., 0.));
        assert!(!eq_f32(0., f32::NAN));
        assert!(!eq_f32(0., 1.));
        assert!(eq_f32(1., 1.));
    }
}
