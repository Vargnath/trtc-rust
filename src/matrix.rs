use crate::float_eq;
use std::ops::{Index, IndexMut};

#[derive(Debug)]
pub struct Matrix2 {
    rows: [[f64; 2]; 2],
}

#[derive(Debug)]
pub struct Matrix3 {
    rows: [[f64; 3]; 3],
}

#[derive(Debug)]
pub struct Matrix4 {
    rows: [[f64; 4]; 4],
}

macro_rules! impl_matrix {
    ($MatrixN:ident, $n:expr) => {
        impl $MatrixN {
            pub fn new(rows: [[f64; $n]; $n]) -> Self {
                Self { rows }
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
    };
}

impl_matrix!(Matrix2, 2);
impl_matrix!(Matrix3, 3);
impl_matrix!(Matrix4, 4);

#[cfg(test)]
mod tests {
    use crate::float_eq;
    use crate::matrix::{Matrix2, Matrix3, Matrix4};

    #[test]
    fn constructing_and_inspecting_a_2_x_2_matrix() {
        let m = Matrix2::new([[-3.0, 5.0], [1.0, -2.0]]);

        assert!(float_eq(m[0][0], -3.0));
        assert!(float_eq(m[0][1], 5.0));
        assert!(float_eq(m[1][0], 1.0));
        assert!(float_eq(m[1][1], -2.0));
    }

    #[test]
    fn constructing_and_inspecting_a_3_x_3_matrix() {
        let m = Matrix3::new([[-3.0, 5.0, 0.0], [1.0, -2.0, -7.0], [0.0, 1.0, 1.0]]);

        assert!(float_eq(m[0][0], -3.0));
        assert!(float_eq(m[1][1], -2.0));
        assert!(float_eq(m[2][2], 1.0));
    }

    #[test]
    fn constructing_and_inspecting_a_4_x_4_matrix() {
        let m = Matrix4::new([
            [1.0, 2.0, 3.0, 4.0],
            [5.5, 6.5, 7.5, 8.5],
            [9.0, 10.0, 11.0, 12.0],
            [13.5, 14.5, 15.5, 16.5],
        ]);

        assert!(float_eq(m[0][0], 1.0));
        assert!(float_eq(m[0][3], 4.0));
        assert!(float_eq(m[1][0], 5.5));
        assert!(float_eq(m[1][2], 7.5));
        assert!(float_eq(m[2][2], 11.0));
        assert!(float_eq(m[3][0], 13.5));
        assert!(float_eq(m[3][2], 15.5));
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
}
