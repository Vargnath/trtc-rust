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

macro_rules! impl_submatrix {
    ($MatrixN:ident, $n:expr, $SubMatrixN:ident) => {
        impl $MatrixN {
            pub fn submatrix(&self, row: usize, column: usize) -> $SubMatrixN {
                let mut result = $SubMatrixN::default();
                self.rows[..row]
                    .iter()
                    .chain(self.rows[row + 1..].iter())
                    .enumerate()
                    .for_each(|(i, columns)| {
                        columns[..column]
                            .iter()
                            .chain(columns[column + 1..].iter())
                            .enumerate()
                            .for_each(|(j, element)| {
                                result[i][j] = *element;
                            });
                    });
                result
            }

            pub fn minor(&self, row: usize, column: usize) -> f64 {
                self.submatrix(row, column).determinant()
            }

            pub fn cofactor(&self, row: usize, column: usize) -> f64 {
                let minor = self.minor(row, column);
                if (row + column) % 2 == 0 {
                    minor
                } else {
                    -minor
                }
            }

            pub fn determinant(&self) -> f64 {
                let mut determinant: f64 = 0.0;

                for i in 0..$n {
                    determinant += self.rows[0][i] * self.cofactor(0, i);
                }
                determinant
            }

            pub fn invertible(&self) -> bool {
                self.determinant() != 0.0
            }

            pub fn inverse(&self) -> Self {
                if !self.invertible() {
                    panic!("matrix is not invertible");
                }
                let mut result = $MatrixN::default();
                let determinant = self.determinant();
                for row in 0..$n {
                    for col in 0..$n {
                        let cofactor = self.cofactor(row, col);
                        result[col][row] = cofactor / determinant;
                    }
                }
                result
            }
        }
    };
}

impl_submatrix!(Matrix3, 3, Matrix2);
impl_submatrix!(Matrix4, 4, Matrix3);

impl Matrix2 {
    pub fn determinant(&self) -> f64 {
        self[0][0] * self[1][1] - self[0][1] * self[1][0]
    }
}

impl Matrix4 {
    pub fn identity() -> Self {
        Matrix4::new([
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ])
    }

    pub fn translation(x: f64, y: f64, z: f64) -> Self {
        let mut translation = Self::identity();
        translation[0][3] = x;
        translation[1][3] = y;
        translation[2][3] = z;
        translation
    }

    pub fn translate(self, x: f64, y: f64, z: f64) -> Self {
        Self::translation(x, y, z) * self
    }

    pub fn scaling(x: f64, y: f64, z: f64) -> Self {
        let mut scaling = Self::identity();
        scaling[0][0] = x;
        scaling[1][1] = y;
        scaling[2][2] = z;
        scaling
    }

    pub fn scale(self, x: f64, y: f64, z: f64) -> Self {
        Self::scaling(x, y, z) * self
    }

    pub fn rotation_x(r: f64) -> Self {
        let mut rotation = Self::identity();
        rotation[1][1] = r.cos();
        rotation[1][2] = -r.sin();
        rotation[2][1] = r.sin();
        rotation[2][2] = r.cos();
        rotation
    }

    pub fn rotate_x(self, r: f64) -> Self {
        Self::rotation_x(r) * self
    }

    pub fn rotation_y(r: f64) -> Self {
        let mut rotation = Self::identity();
        rotation[0][0] = r.cos();
        rotation[0][2] = r.sin();
        rotation[2][0] = -r.sin();
        rotation[2][2] = r.cos();
        rotation
    }

    pub fn rotate_y(self, r: f64) -> Self {
        Self::rotation_y(r) * self
    }

    pub fn rotation_z(r: f64) -> Self {
        let mut rotation = Self::identity();
        rotation[0][0] = r.cos();
        rotation[0][1] = -r.sin();
        rotation[1][0] = r.sin();
        rotation[1][1] = r.cos();
        rotation
    }

    pub fn rotate_z(self, r: f64) -> Self {
        Self::rotation_z(r) * self
    }

    pub fn shearing(xy: f64, xz: f64, yx: f64, yz: f64, zx: f64, zy: f64) -> Self {
        let mut shearing = Self::identity();
        shearing[0][1] = xy;
        shearing[0][2] = xz;
        shearing[1][0] = yx;
        shearing[1][2] = yz;
        shearing[2][0] = zx;
        shearing[2][1] = zy;
        shearing
    }

    pub fn shear(self, xy: f64, xz: f64, yx: f64, yz: f64, zx: f64, zy: f64) -> Self {
        Self::shearing(xy, xz, yx, yz, zx, zy) * self
    }

    pub fn view_transform(from: Tuple, to: Tuple, up: Tuple) -> Self {
        let forward = (to - from).normalize();
        let upn = up.normalize();
        let left = forward.cross(upn);
        let true_up = left.cross(forward);

        let orientation = Matrix4::new([
            [left.x, left.y, left.z, 0.0],
            [true_up.x, true_up.y, true_up.z, 0.0],
            [-forward.x, -forward.y, -forward.z, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ]);
        orientation * Matrix4::translation(-from.x, -from.y, -from.z)
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
    use std::f64::consts::PI;

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

    #[test]
    fn calculating_the_determinant_of_a_2_x_2_matrix() {
        let a = Matrix2::new([[1.0, 5.0], [-3.0, 2.0]]);

        assert_float_eq!(a.determinant(), 17.0);
    }

    #[test]
    fn a_submatrix_of_a_3_x_3_matrix_is_a_2_x_2_matrix() {
        let a = Matrix3::new([[1.0, 5.0, 0.0], [-3.0, 2.0, 7.0], [0.0, 6.0, -3.0]]);
        let expected = Matrix2::new([[-3.0, 2.0], [0.0, 6.0]]);

        assert_eq!(a.submatrix(0, 2), expected);
    }

    #[test]
    fn a_submatrix_of_a_4_x_4_matrix_is_a_3_x_3_matrix() {
        let a = Matrix4::new([
            [-6.0, 1.0, 1.0, 6.0],
            [-8.0, 5.0, 8.0, 6.0],
            [-1.0, 0.0, 8.0, 2.0],
            [-7.0, 1.0, -1.0, 1.0],
        ]);
        let expected = Matrix3::new([[-6.0, 1.0, 6.0], [-8.0, 8.0, 6.0], [-7.0, -1.0, 1.0]]);

        assert_eq!(a.submatrix(2, 1), expected);
    }

    #[test]
    fn calculating_a_minor_of_a_3_x_3_matrix() {
        let a = Matrix3::new([[3.0, 5.0, 0.0], [2.0, -1.0, -7.0], [6.0, -1.0, 5.0]]);
        let b = a.submatrix(1, 0);

        assert_float_eq!(b.determinant(), 25.0);
        assert_float_eq!(a.minor(1, 0), 25.0);
    }

    #[test]
    fn calculating_a_cofactor_of_a_3_x_3_matrix() {
        let a = Matrix3::new([[3.0, 5.0, 0.0], [2.0, -1.0, -7.0], [6.0, -1.0, 5.0]]);

        assert_float_eq!(a.minor(0, 0), -12.0);
        assert_float_eq!(a.cofactor(0, 0), -12.0);
        assert_float_eq!(a.minor(1, 0), 25.0);
        assert_float_eq!(a.cofactor(1, 0), -25.0);
    }

    #[test]
    fn calculating_the_determinant_of_a_3_x_3_matrix() {
        let a = Matrix3::new([[1.0, 2.0, 6.0], [-5.0, 8.0, -4.0], [2.0, 6.0, 4.0]]);

        assert_float_eq!(a.cofactor(0, 0), 56.0);
        assert_float_eq!(a.cofactor(0, 1), 12.0);
        assert_float_eq!(a.cofactor(0, 2), -46.0);
        assert_float_eq!(a.determinant(), -196.0);
    }

    #[test]
    fn calculating_the_determinant_of_a_4_x_4_matrix() {
        let a = Matrix4::new([
            [-2.0, -8.0, 3.0, 5.0],
            [-3.0, 1.0, 7.0, 3.0],
            [1.0, 2.0, -9.0, 6.0],
            [-6.0, 7.0, 7.0, -9.0],
        ]);

        assert_float_eq!(a.cofactor(0, 0), 690.0);
        assert_float_eq!(a.cofactor(0, 1), 447.0);
        assert_float_eq!(a.cofactor(0, 2), 210.0);
        assert_float_eq!(a.cofactor(0, 3), 51.0);
        assert_float_eq!(a.determinant(), -4071.0);
    }

    #[test]
    fn testing_an_invertible_matrix_for_invertibility() {
        let a = Matrix4::new([
            [6.0, 4.0, 4.0, 4.0],
            [5.0, 5.0, 7.0, 6.0],
            [4.0, -9.0, 3.0, -7.0],
            [9.0, 1.0, 7.0, -6.0],
        ]);

        assert_float_eq!(a.determinant(), -2120.0);
        assert!(a.invertible());
    }

    #[test]
    fn testing_a_noninvertible_matrix_for_invertibility() {
        let a = Matrix4::new([
            [-4.0, 2.0, -2.0, -3.0],
            [9.0, 6.0, 2.0, 6.0],
            [0.0, -5.0, 1.0, -5.0],
            [0.0, 0.0, 0.0, 0.0],
        ]);

        assert_float_eq!(a.determinant(), 0.0);
        assert!(!a.invertible());
    }

    #[test]
    fn calculating_the_inverse_of_a_matrix() {
        let a = Matrix4::new([
            [-5.0, 2.0, 6.0, -8.0],
            [1.0, -5.0, 1.0, 8.0],
            [7.0, 7.0, -6.0, -7.0],
            [1.0, -3.0, 7.0, 4.0],
        ]);
        let b = a.inverse();
        let expected = Matrix4::new([
            [0.21805, 0.45113, 0.24060, -0.04511],
            [-0.80827, -1.45677, -0.44361, 0.52068],
            [-0.07895, -0.22368, -0.05263, 0.19737],
            [-0.52256, -0.81391, -0.30075, 0.30639],
        ]);

        assert_float_eq!(a.determinant(), 532.0);
        assert_float_eq!(a.cofactor(2, 3), -160.0);
        assert_float_eq!(b[3][2], -160.0 / 532.0);
        assert_float_eq!(a.cofactor(3, 2), 105.0);
        assert_float_eq!(b[2][3], 105.0 / 532.0);
        assert_eq!(b, expected);
    }

    #[test]
    fn calculating_the_inverse_of_another_matrix() {
        let a = Matrix4::new([
            [8.0, -5.0, 9.0, 2.0],
            [7.0, 5.0, 6.0, 1.0],
            [-6.0, 0.0, 9.0, 6.0],
            [-3.0, 0.0, -9.0, -4.0],
        ]);
        let expected = Matrix4::new([
            [-0.15385, -0.15385, -0.28205, -0.53846],
            [-0.07692, 0.12308, 0.02564, 0.03077],
            [0.35897, 0.35897, 0.43590, 0.92308],
            [-0.69231, -0.69231, -0.76923, -1.92308],
        ]);

        assert_eq!(a.inverse(), expected);
    }

    #[test]
    fn calculating_the_inverse_of_a_third_matrix() {
        let a = Matrix4::new([
            [9.0, 3.0, 0.0, 9.0],
            [-5.0, -2.0, -6.0, -3.0],
            [-4.0, 9.0, 6.0, 4.0],
            [-7.0, 6.0, 6.0, 2.0],
        ]);
        let expected = Matrix4::new([
            [-0.04074, -0.07778, 0.14444, -0.22222],
            [-0.07778, 0.03333, 0.36667, -0.33333],
            [-0.02901, -0.14630, -0.10926, 0.12963],
            [0.17778, 0.06667, -0.26667, 0.33333],
        ]);

        assert_eq!(a.inverse(), expected);
    }

    #[test]
    fn multiplying_a_product_by_its_inverse() {
        let a = Matrix4::new([
            [3.0, -9.0, 7.0, 3.0],
            [3.0, -8.0, 2.0, -9.0],
            [-4.0, 4.0, 4.0, 1.0],
            [-6.0, 5.0, -1.0, 1.0],
        ]);
        let b = Matrix4::new([
            [8.0, 2.0, 2.0, 2.0],
            [3.0, -1.0, 7.0, 0.0],
            [7.0, 0.0, 5.0, 4.0],
            [6.0, -2.0, 0.0, 5.0],
        ]);
        let c = a * b;

        assert_eq!(c * b.inverse(), a);
    }

    #[test]
    fn multiplying_by_a_translation_matrix() {
        let transform = Matrix4::translation(5.0, -3.0, 2.0);
        let p = Tuple::new_point(-3.0, 4.0, 5.0);
        let expected = Tuple::new_point(2.0, 1.0, 7.0);

        assert_eq!(transform * p, expected);
    }

    #[test]
    fn multiplying_by_the_inverse_of_a_translation_matrix() {
        let transform = Matrix4::translation(5.0, -3.0, 2.0);
        let inv = transform.inverse();
        let p = Tuple::new_point(-3.0, 4.0, 5.0);
        let expected = Tuple::new_point(-8.0, 7.0, 3.0);

        assert_eq!(inv * p, expected);
    }

    #[test]
    fn translation_does_not_affect_vectors() {
        let transform = Matrix4::translation(5.0, -3.0, 2.0);
        let v = Tuple::new_vector(-3.0, 4.0, 5.0);

        assert_eq!(transform * v, v);
    }

    #[test]
    fn a_scaling_matrix_applied_to_a_point() {
        let transform = Matrix4::scaling(2.0, 3.0, 4.0);
        let p = Tuple::new_point(-4.0, 6.0, 8.0);
        let expected = Tuple::new_point(-8.0, 18.0, 32.0);

        assert_eq!(transform * p, expected);
    }

    #[test]
    fn a_scaling_matrix_applied_to_a_vector() {
        let transform = Matrix4::scaling(2.0, 3.0, 4.0);
        let v = Tuple::new_vector(-4.0, 6.0, 8.0);
        let expected = Tuple::new_vector(-8.0, 18.0, 32.0);

        assert_eq!(transform * v, expected);
    }

    #[test]
    fn multiplying_by_the_inverse_of_a_scaling_matrix() {
        let transform = Matrix4::scaling(2.0, 3.0, 4.0);
        let inv = transform.inverse();
        let v = Tuple::new_vector(-4.0, 6.0, 8.0);
        let expected = Tuple::new_vector(-2.0, 2.0, 2.0);

        assert_eq!(inv * v, expected);
    }

    #[test]
    fn reflection_is_scaling_by_a_negative_value() {
        let transform = Matrix4::scaling(-1.0, 1.0, 1.0);
        let p = Tuple::new_point(2.0, 3.0, 4.0);
        let expected = Tuple::new_point(-2.0, 3.0, 4.0);

        assert_eq!(transform * p, expected);
    }

    #[test]
    fn rotating_a_point_around_the_x_axis() {
        let p = Tuple::new_point(0.0, 1.0, 0.0);
        let half_quarter = Matrix4::rotation_x(PI / 4.0);
        let full_quarter = Matrix4::rotation_x(PI / 2.0);

        let expected = Tuple::new_point(0.0, f64::sqrt(2.0) / 2.0, f64::sqrt(2.0) / 2.0);
        assert_eq!(half_quarter * p, expected);

        let expected = Tuple::new_point(0.0, 0.0, 1.0);
        assert_eq!(full_quarter * p, expected);
    }

    #[test]
    fn the_inverse_of_an_x_rotation_rotates_in_the_opposite_direction() {
        let p = Tuple::new_point(0.0, 1.0, 0.0);
        let half_quarter = Matrix4::rotation_x(PI / 4.0);
        let inv = half_quarter.inverse();
        let expected = Tuple::new_point(0.0, f64::sqrt(2.0) / 2.0, -f64::sqrt(2.0) / 2.0);

        assert_eq!(inv * p, expected);
    }

    #[test]
    fn rotating_a_point_around_the_y_axis() {
        let p = Tuple::new_point(0.0, 0.0, 1.0);
        let half_quarter = Matrix4::rotation_y(PI / 4.0);
        let full_quarter = Matrix4::rotation_y(PI / 2.0);

        let expected = Tuple::new_point(f64::sqrt(2.0) / 2.0, 0.0, f64::sqrt(2.0) / 2.0);
        assert_eq!(half_quarter * p, expected);

        let expected = Tuple::new_point(1.0, 0.0, 0.0);
        assert_eq!(full_quarter * p, expected);
    }

    #[test]
    fn rotating_a_point_around_the_z_axis() {
        let p = Tuple::new_point(0.0, 1.0, 0.0);
        let half_quarter = Matrix4::rotation_z(PI / 4.0);
        let full_quarter = Matrix4::rotation_z(PI / 2.0);

        let expected = Tuple::new_point(-f64::sqrt(2.0) / 2.0, f64::sqrt(2.0) / 2.0, 0.0);
        assert_eq!(half_quarter * p, expected);

        let expected = Tuple::new_point(-1.0, 0.0, 0.0);
        assert_eq!(full_quarter * p, expected);
    }

    #[test]
    fn a_shearing_transformation_moves_x_in_proportion_to_y() {
        let transform = Matrix4::shearing(1.0, 0.0, 0.0, 0.0, 0.0, 0.0);
        let p = Tuple::new_point(2.0, 3.0, 4.0);
        let expected = Tuple::new_point(5.0, 3.0, 4.0);

        assert_eq!(transform * p, expected);
    }

    #[test]
    fn a_shearing_transformation_moves_x_in_proportion_to_z() {
        let transform = Matrix4::shearing(0.0, 1.0, 0.0, 0.0, 0.0, 0.0);
        let p = Tuple::new_point(2.0, 3.0, 4.0);
        let expected = Tuple::new_point(6.0, 3.0, 4.0);

        assert_eq!(transform * p, expected);
    }

    #[test]
    fn a_shearing_transformation_moves_y_in_proportion_to_x() {
        let transform = Matrix4::shearing(0.0, 0.0, 1.0, 0.0, 0.0, 0.0);
        let p = Tuple::new_point(2.0, 3.0, 4.0);
        let expected = Tuple::new_point(2.0, 5.0, 4.0);

        assert_eq!(transform * p, expected);
    }

    #[test]
    fn a_shearing_transformation_moves_y_in_proportion_to_z() {
        let transform = Matrix4::shearing(0.0, 0.0, 0.0, 1.0, 0.0, 0.0);
        let p = Tuple::new_point(2.0, 3.0, 4.0);
        let expected = Tuple::new_point(2.0, 7.0, 4.0);

        assert_eq!(transform * p, expected);
    }

    #[test]
    fn a_shearing_transformation_moves_z_in_proportion_to_x() {
        let transform = Matrix4::shearing(0.0, 0.0, 0.0, 0.0, 1.0, 0.0);
        let p = Tuple::new_point(2.0, 3.0, 4.0);
        let expected = Tuple::new_point(2.0, 3.0, 6.0);

        assert_eq!(transform * p, expected);
    }

    #[test]
    fn a_shearing_transformation_moves_z_in_proportion_to_y() {
        let transform = Matrix4::shearing(0.0, 0.0, 0.0, 0.0, 0.0, 1.0);
        let p = Tuple::new_point(2.0, 3.0, 4.0);
        let expected = Tuple::new_point(2.0, 3.0, 7.0);

        assert_eq!(transform * p, expected);
    }

    #[test]
    fn individual_transformations_are_applied_in_sequence() {
        let p = Tuple::new_point(1.0, 0.0, 1.0);
        let a = Matrix4::rotation_x(PI / 2.0);
        let b = Matrix4::scaling(5.0, 5.0, 5.0);
        let c = Matrix4::translation(10.0, 5.0, 7.0);

        let p2 = a * p;
        let expected = Tuple::new_point(1.0, -1.0, 0.0);
        assert_eq!(p2, expected);

        let p3 = b * p2;
        let expected = Tuple::new_point(5.0, -5.0, 0.0);
        assert_eq!(p3, expected);

        let p4 = c * p3;
        let expected = Tuple::new_point(15.0, 0.0, 7.0);
        assert_eq!(p4, expected);
    }

    #[test]
    fn chained_transformations_must_be_applied_in_reverse_order() {
        let p = Tuple::new_point(1.0, 0.0, 1.0);
        let a = Matrix4::rotation_x(PI / 2.0);
        let b = Matrix4::scaling(5.0, 5.0, 5.0);
        let c = Matrix4::translation(10.0, 5.0, 7.0);
        let t = c * b * a;

        let expected = Tuple::new_point(15.0, 0.0, 7.0);
        assert_eq!(t * p, expected);
    }

    #[test]
    fn fluent_api_applies_transformation_in_sequence() {
        let p = Tuple::new_point(1.0, 0.0, 1.0);
        let transform = Matrix4::identity()
            .rotate_x(PI / 2.0)
            .scale(5.0, 5.0, 5.0)
            .translate(10.0, 5.0, 7.0);

        let expected = Tuple::new_point(15.0, 0.0, 7.0);
        assert_eq!(transform * p, expected);
    }

    #[test]
    fn the_transformation_matrix_for_the_default_orientation() {
        let from = Tuple::new_point(0.0, 0.0, 0.0);
        let to = Tuple::new_point(0.0, 0.0, -1.0);
        let up = Tuple::new_vector(0.0, 1.0, 0.0);
        let t = Matrix4::view_transform(from, to, up);

        assert_eq!(t, Matrix4::identity());
    }

    #[test]
    fn a_view_transformation_matrix_looking_in_positive_z_direction() {
        let from = Tuple::new_point(0.0, 0.0, 0.0);
        let to = Tuple::new_point(0.0, 0.0, 1.0);
        let up = Tuple::new_vector(0.0, 1.0, 0.0);
        let t = Matrix4::view_transform(from, to, up);

        assert_eq!(t, Matrix4::scaling(-1.0, 1.0, -1.0));
    }

    #[test]
    fn the_view_transformation_moves_the_world() {
        let from = Tuple::new_point(0.0, 0.0, 8.0);
        let to = Tuple::new_point(0.0, 0.0, 0.0);
        let up = Tuple::new_vector(0.0, 1.0, 0.0);
        let t = Matrix4::view_transform(from, to, up);

        assert_eq!(t, Matrix4::translation(0.0, 0.0, -8.0));
    }

    #[test]
    fn an_arbitrary_view_transformation() {
        let from = Tuple::new_point(1.0, 3.0, 2.0);
        let to = Tuple::new_point(4.0, -2.0, 8.0);
        let up = Tuple::new_vector(1.0, 1.0, 0.0);
        let t = Matrix4::view_transform(from, to, up);

        let expected = Matrix4::new([
            [-0.50709, 0.50709, 0.67612, -2.36643],
            [0.76772, 0.60609, 0.12122, -2.82843],
            [-0.35857, 0.59761, -0.71714, 0.00000],
            [0.00000, 0.00000, 0.00000, 1.00000],
        ]);
        assert_eq!(t, expected);
    }
}
