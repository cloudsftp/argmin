use crate::ArgminMul;
use faer::{unzip, zip, Col, ColMut, ColRef};
use faer_traits::{ComplexField, MulByRef};

/// ColRef * Scalar -> Col
impl<E: ComplexField> ArgminMul<E, Col<E>> for ColRef<'_, E> {
    #[inline]
    fn mul(&self, other: &E) -> Col<E> {
        let mut result = Col::zeros(self.nrows()); // TODO: why not possible like sub
        zip!(&mut result, self).for_each(|unzip!(result, self_)| *result = self_.mul_by_ref(other));
        result
    }
}

/// Scalar * ColRef -> Col
impl<E: ComplexField> ArgminMul<ColRef<'_, E>, Col<E>> for E {
    #[inline]
    fn mul(&self, other: &ColRef<'_, E>) -> Col<E> {
        let mut result = Col::zeros(other.nrows()); // TODO: why not possible like sub
        zip!(&mut result, other).for_each(|unzip!(result, other)| *result = self.mul_by_ref(other));
        result
    }
}

/// Col * Scalar -> Col
impl<E: ComplexField> ArgminMul<E, Col<E>> for Col<E> {
    #[inline]
    fn mul(&self, other: &E) -> Col<E> {
        <_ as ArgminMul<_, _>>::mul(&self.as_ref(), other)
    }
}

/// Scalar * Col -> Col
impl<E: ComplexField> ArgminMul<Col<E>, Col<E>> for E {
    #[inline]
    fn mul(&self, other: &Col<E>) -> Col<E> {
        <_ as ArgminMul<_, _>>::mul(self, &other.as_ref())
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
