use crate::float_eq;
use crate::tuple::Tuple;
use std::ops::{Index, IndexMut, Mul};

#[derive(Debug, Default, Copy, Clone)]
pub struct Matrix2 {
    rows: [[f64; 2]; 2],
}

#[derive(Debug, Default, Copy, Clone)]
pub struct Matrix3 {
    rows: [[f64; 3]; 3],
}

#[derive(Debug, Default, Copy, Clone)]
pub struct Matrix4 {
    rows: [[f64; 4]; 4],
}

macro_rules! impl_matrix {
    ($MatrixN:ident, $n:expr) => {
        impl $MatrixN {
            pub fn new(rows: [[f64; $n]; $n]) -> Self {
                Self { rows }
            }

            pub fn transpose(&self) -> Self {
                let mut result = Self::default();
                for i in 0..$n {
                    for j in 0..$n {
                        result[i][j] = self[j][i];
                    }
                }
                result
            }
        }

        impl PartialEq for $MatrixN {
            fn eq(&self, other: &Self) -> bool {
                self.rows
                    .iter()
                    .flatten()
                    .zip(other.rows.iter().flatten())
                    .all(|(lhs, rhs)| float_eq(*lhs, *rhs))
            }
        }

        impl Index<usize> for $MatrixN {
            type Output = [f64; $n];

            fn index(&self, index: usize) -> &Self::Output {
                &self.rows[index]
            }
        }

        impl IndexMut<usize> for $MatrixN {
            fn index_mut(&mut self, index: usize) -> &mut Self::Output {
                &mut self.rows[index]
            }
        }

        impl Mul for $MatrixN {
            type Output = $MatrixN;

            fn mul(self, rhs: Self) -> Self::Output {
                let mut result = Self::Output::default();
                for i in 0..$n {
                    for j in 0..$n {
                        let element = &mut result[i][j];
                        for k in 0..$n {
                            *element += self.rows[i][k] * rhs.rows[k][j];
                        }
                    }
                }
                result
            }
        }
    };
}

impl_matrix!(Matrix2, 2);
impl_matrix!(Matrix3, 3);
impl_matrix!(Matrix4, 4);

impl Matrix4 {
    pub fn identity() -> Self {
        Matrix4::new([
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ])
    }
}

impl Mul<Tuple> for Matrix4 {
    type Output = Tuple;

    fn mul(self, rhs: Tuple) -> Self::Output {
        Tuple::new(
            self[0][0] * rhs.x + self[0][1] * rhs.y + self[0][2] * rhs.z + self[0][3] * rhs.w,
            self[1][0] * rhs.x + self[1][1] * rhs.y + self[1][2] * rhs.z + self[1][3] * rhs.w,
            self[2][0] * rhs.x + self[2][1] * rhs.y + self[2][2] * rhs.z + self[2][3] * rhs.w,
            self[3][0] * rhs.x + self[3][1] * rhs.y + self[3][2] * rhs.z + self[3][3] * rhs.w,
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::assert_float_eq;
    use crate::matrix::{Matrix2, Matrix3, Matrix4};
    use crate::tuple::Tuple;

    #[test]
    fn constructing_and_inspecting_a_2_x_2_matrix() {
        let m = Matrix2::new([[-3.0, 5.0], [1.0, -2.0]]);

        assert_float_eq!(m[0][0], -3.0);
        assert_float_eq!(m[0][1], 5.0);
        assert_float_eq!(m[1][0], 1.0);
        assert_float_eq!(m[1][1], -2.0);
    }

    #[test]
    fn constructing_and_inspecting_a_3_x_3_matrix() {
        let m = Matrix3::new([[-3.0, 5.0, 0.0], [1.0, -2.0, -7.0], [0.0, 1.0, 1.0]]);

        assert_float_eq!(m[0][0], -3.0);
        assert_float_eq!(m[1][1], -2.0);
        assert_float_eq!(m[2][2], 1.0);
    }

    #[test]
    fn constructing_and_inspecting_a_4_x_4_matrix() {
        let m = Matrix4::new([
            [1.0, 2.0, 3.0, 4.0],
            [5.5, 6.5, 7.5, 8.5],
            [9.0, 10.0, 11.0, 12.0],
            [13.5, 14.5, 15.5, 16.5],
        ]);

        assert_float_eq!(m[0][0], 1.0);
        assert_float_eq!(m[0][3], 4.0);
        assert_float_eq!(m[1][0], 5.5);
        assert_float_eq!(m[1][2], 7.5);
        assert_float_eq!(m[2][2], 11.0);
        assert_float_eq!(m[3][0], 13.5);
        assert_float_eq!(m[3][2], 15.5);
    }

    #[test]
    fn matrix_equality_with_identical_matrices() {
        let a = Matrix4::new([
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 8.0, 7.0, 6.0],
            [5.0, 4.0, 3.0, 2.0],
        ]);
        let b = Matrix4::new([
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 8.0, 7.0, 6.0],
            [5.0, 4.0, 3.0, 2.0],
        ]);

        assert_eq!(a, b);
    }

    #[test]
    fn matrix_equality_with_different_matrices() {
        let a = Matrix4::new([
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 8.0, 7.0, 6.0],
            [5.0, 4.0, 3.0, 2.0],
        ]);
        let b = Matrix4::new([
            [3.0, 4.0, 5.0, 6.0],
            [7.0, 8.0, 9.0, 8.0],
            [7.0, 6.0, 5.0, 4.0],
            [3.0, 2.0, 1.0, 2.0],
        ]);

        assert_ne!(a, b);
    }

    #[test]
    fn multiplying_two_matrices() {
        let a = Matrix4::new([
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 8.0, 7.0, 6.0],
            [5.0, 4.0, 3.0, 2.0],
        ]);
        let b = Matrix4::new([
            [-2.0, 1.0, 2.0, 3.0],
            [3.0, 2.0, 1.0, -1.0],
            [4.0, 3.0, 6.0, 5.0],
            [1.0, 2.0, 7.0, 8.0],
        ]);
        let expected = Matrix4::new([
            [20.0, 22.0, 50.0, 48.0],
            [44.0, 54.0, 114.0, 108.0],
            [40.0, 58.0, 110.0, 102.0],
            [16.0, 26.0, 46.0, 42.0],
        ]);

        assert_eq!(a * b, expected);
    }

    #[test]
    fn a_matrix_multiplied_by_a_tuple() {
        let a = Matrix4::new([
            [1.0, 2.0, 3.0, 4.0],
            [2.0, 4.0, 4.0, 2.0],
            [8.0, 6.0, 4.0, 1.0],
            [0.0, 0.0, 0.0, 1.0],
        ]);
        let b = Tuple::new(1.0, 2.0, 3.0, 1.0);
        let expected = Tuple::new(18.0, 24.0, 33.0, 1.0);

        assert_eq!(a * b, expected);
    }

    #[test]
    fn multiplying_a_matrix_by_the_identity_matrix() {
        let a = Matrix4::new([
            [0.0, 1.0, 2.0, 4.0],
            [1.0, 2.0, 4.0, 8.0],
            [2.0, 4.0, 8.0, 16.0],
            [4.0, 8.0, 16.0, 32.0],
        ]);

        assert_eq!(a * Matrix4::identity(), a);
    }

    #[test]
    fn multiplying_the_identity_matrix_by_a_tuple() {
        let a = Tuple::new(1.0, 2.0, 3.0, 4.0);

        assert_eq!(Matrix4::identity() * a, a);
    }

    #[test]
    fn transposing_a_matrix() {
        let a = Matrix4::new([
            [0.0, 9.0, 3.0, 0.0],
            [9.0, 8.0, 0.0, 8.0],
            [1.0, 8.0, 5.0, 3.0],
            [0.0, 0.0, 5.0, 8.0],
        ]);
        let expected = Matrix4::new([
            [0.0, 9.0, 1.0, 0.0],
            [9.0, 8.0, 8.0, 0.0],
            [3.0, 0.0, 5.0, 5.0],
            [0.0, 8.0, 3.0, 8.0],
        ]);

        assert_eq!(a.transpose(), expected);
    }

    #[test]
    fn transposing_the_identity_matrix() {
        let a = Matrix4::identity().transpose();

        assert_eq!(a, Matrix4::identity());
    }
}
