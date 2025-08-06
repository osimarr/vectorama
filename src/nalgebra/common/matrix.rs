use crate::Matrix;

use super::na;

impl<const M: usize, const N: usize>
    From<&na::Matrix<f32, na::Const<M>, na::Const<N>, na::ArrayStorage<f32, M, N>>>
    for Matrix<M, N>
{
    fn from(
        value: &na::Matrix<f32, na::Const<M>, na::Const<N>, na::ArrayStorage<f32, M, N>>,
    ) -> Self {
        Matrix::from_flattened(value.as_ref())
    }
}

impl<const M: usize, const N: usize>
    From<na::Matrix<f32, na::Const<M>, na::Const<N>, na::ArrayStorage<f32, M, N>>>
    for Matrix<M, N>
{
    fn from(
        value: na::Matrix<f32, na::Const<M>, na::Const<N>, na::ArrayStorage<f32, M, N>>,
    ) -> Self {
        Matrix::from_flattened(value.as_ref())
    }
}

impl<const M: usize, const N: usize> From<&Matrix<M, N>>
    for na::Matrix<f32, na::Const<M>, na::Const<N>, na::ArrayStorage<f32, M, N>>
{
    fn from(value: &Matrix<M, N>) -> Self {
        na::Matrix::<f32, na::Const<M>, na::Const<N>, na::ArrayStorage<f32, M, N>>::from_column_slice(value.as_flattened())
    }
}

impl<const M: usize, const N: usize> From<Matrix<M, N>>
    for na::Matrix<f32, na::Const<M>, na::Const<N>, na::ArrayStorage<f32, M, N>>
{
    fn from(value: Matrix<M, N>) -> Self {
        na::Matrix::<f32, na::Const<M>, na::Const<N>, na::ArrayStorage<f32, M, N>>::from_column_slice(value.as_flattened())
    }
}
