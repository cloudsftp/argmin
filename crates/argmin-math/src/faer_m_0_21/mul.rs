use crate::ArgminMul;
use faer::{
    mat::{AsMatMut, AsMatRef},
    reborrow::{IntoConst, Reborrow, ReborrowMut},
    unzip, zip, Col, ColMut, ColRef, Mat, MatMut, MatRef,
};
use faer_traits::{ComplexField, MulByRef};
use std::ops::Mul;

/// MatRef * Scalar -> Mat
impl<E> ArgminMul<E, Mat<E>> for MatRef<'_, E>
where
    E: ComplexField,
{
    #[inline]
    fn mul(&self, other: &E) -> Mat<E> {
        faer::zip!(self).map(|faer::unzip!(this)| this.mul_by_ref(other))
    }
}

/// Scalar * MatRef-> Mat
impl<'a, E> ArgminMul<MatRef<'a, E>, Mat<E>> for E
where
    E: ComplexField,
{
    #[inline]
    fn mul(&self, other: &MatRef<'a, E>) -> Mat<E> {
        // commutative with MatRef + Scalar so we can fall back on that case
        <_ as ArgminMul<_, _>>::mul(other, self)
    }
}

/// Mat * Scalar -> Mat
impl<E> ArgminMul<E, Mat<E>> for Mat<E>
where
    E: ComplexField,
{
    #[inline]
    fn mul(&self, other: &E) -> Mat<E> {
        //@note(geo-ant) because we are taking self by reference we
        // cannot mutate the matrix in place, so we can just as well
        // reuse the reference code
        <_ as ArgminMul<_, _>>::mul(&self.as_mat_ref(), other)
    }
}

/// Scalar * Mat -> Mat
impl<E> ArgminMul<Mat<E>, Mat<E>> for E
where
    E: ComplexField,
{
    #[inline]
    fn mul(&self, other: &Mat<E>) -> Mat<E> {
        // commutative with Mat * Scalar so we can fall back on that case
        <_ as ArgminMul<_, _>>::mul(other, self)
    }
}

/// MatRef * MatRef -> Mat (pointwise multiplication)
impl<'a, E: ComplexField> ArgminMul<MatRef<'a, E>, Mat<E>> for MatRef<'_, E> {
    #[inline]
    fn mul(&self, other: &MatRef<'a, E>) -> Mat<E> {
        faer::zip!(self, other).map(|faer::unzip!(this, other)| this.mul_by_ref(other))
    }
}

/// MatRef * Mat -> Mat (pointwise multiplication)
impl<E: ComplexField> ArgminMul<Mat<E>, Mat<E>> for MatRef<'_, E> {
    #[inline]
    fn mul(&self, other: &Mat<E>) -> Mat<E> {
        <_ as ArgminMul<_, _>>::mul(self, &other.as_mat_ref())
    }
}

/// Mat * MatRef-> Mat (pointwise multiplication)
impl<'a, E: ComplexField> ArgminMul<MatRef<'a, E>, Mat<E>> for Mat<E> {
    #[inline]
    fn mul(&self, other: &MatRef<'a, E>) -> Mat<E> {
        <_ as ArgminMul<_, _>>::mul(&self.as_mat_ref(), other)
    }
}

/// Mat * Mat -> Mat (pointwise multiplication)
impl<E: ComplexField> ArgminMul<Mat<E>, Mat<E>> for Mat<E> {
    #[inline]
    fn mul(&self, other: &Mat<E>) -> Mat<E> {
        <_ as ArgminMul<_, _>>::mul(&self.as_mat_ref(), &other.as_mat_ref())
    }
}

/// ColRef * ColRef -> Col (pointwise multiplication)
impl<E: ComplexField> ArgminMul<ColRef<'_, E>, Col<E>> for ColRef<'_, E> {
    #[inline]
    fn mul(&self, other: &ColRef<'_, E>) -> Col<E> {
        let mut result = Col::zeros(self.nrows()); // TODO: why not possible like sub
        zip!(&mut result, self, other)
            .for_each(|unzip!(result, this, other)| *result = this.mul_by_ref(other));
        result
    }
}

/// Col * Col -> Col (pointwise multiplication)
impl<E: ComplexField> ArgminMul<Col<E>, Col<E>> for Col<E> {
    #[inline]
    fn mul(&self, other: &Col<E>) -> Col<E> {
        <_ as ArgminMul<_, _>>::mul(&self.as_ref(), &other.as_ref())
    }
}

/// E * ColRef -> Col
impl<E: ComplexField> ArgminMul<ColRef<'_, E>, Col<E>> for E {
    #[inline]
    fn mul(&self, other: &ColRef<'_, E>) -> Col<E> {
        let mut result = Col::zeros(other.nrows()); // TODO: why not possible like sub
        zip!(&mut result, other).for_each(|unzip!(result, other)| *result = self.mul_by_ref(other));
        result
    }
}

/// E * Col -> Col
impl<E: ComplexField> ArgminMul<Col<E>, Col<E>> for E {
    #[inline]
    fn mul(&self, other: &Col<E>) -> Col<E> {
        <_ as ArgminMul<_, _>>::mul(self, &other.as_ref())
    }
}

/// ColRef * E -> Col
impl<E: ComplexField> ArgminMul<E, Col<E>> for ColRef<'_, E> {
    #[inline]
    fn mul(&self, other: &E) -> Col<E> {
        let mut result = Col::zeros(self.nrows()); // TODO: why not possible like sub
        zip!(&mut result, self).for_each(|unzip!(result, self_)| *result = self_.mul_by_ref(other));
        result
    }
}

/// Col * E -> Col
impl<E: ComplexField> ArgminMul<E, Col<E>> for Col<E> {
    #[inline]
    fn mul(&self, other: &E) -> Col<E> {
        <_ as ArgminMul<_, _>>::mul(&self.as_ref(), other)
    }
}
