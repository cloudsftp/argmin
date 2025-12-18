use crate::ArgminSub;
use faer::{unzip, zip, Col, ColMut, ColRef};
use faer_traits::{ComplexField, Conjugate, SubByRef};
use std::ops::{Sub, SubAssign};

/// ColRef / ColRef -> Col
impl<E: ComplexField> ArgminSub<ColRef<'_, E>, Col<E>> for ColRef<'_, E> {
    #[inline]
    fn sub(&self, other: &ColRef<'_, E>) -> Col<E> {
        zip!(self, other).map(|unzip!(this, other)| this.sub_by_ref(other))
    }
}

/// Col / Col -> Col
impl<E: ComplexField> ArgminSub<Col<E>, Col<E>> for Col<E> {
    #[inline]
    fn sub(&self, other: &Col<E>) -> Col<E> {
        <_ as ArgminSub<_, _>>::sub(&self.as_ref(), &other.as_ref())
    }
}
