use crate::faer_tests::test_helper::*;
use crate::ArgminDot;
use approx::assert_relative_eq;
use faer::mat::AsMatRef;
use faer::Mat;
use paste::item;

macro_rules! make_test {
    ($t:ty) => {
        item! {
            #[test]
            fn [<test_col_col_ $t>]() {
                let a = col3_new(1 as $t, 2 as $t, 3 as $t);
                let b = col3_new(4 as $t, 5 as $t, 6 as $t);
                // all owned and reference type combinations
                let res1: $t = <_ as ArgminDot<_, _>>::dot(&a, &b);
                let res2: $t = <_ as ArgminDot<_, _>>::dot(&a.as_ref(), &b);
                let res3: $t = <_ as ArgminDot<_, _>>::dot(&a, &b.as_ref());
                let res4: $t = <_ as ArgminDot<_, _>>::dot(&a.as_ref(), &b.as_ref());
                assert_relative_eq!(res1 as f64, 32 as f64, epsilon = f64::EPSILON);
                assert_relative_eq!(res2 as f64, 32 as f64, epsilon = f64::EPSILON);
                assert_relative_eq!(res3 as f64, 32 as f64, epsilon = f64::EPSILON);
                assert_relative_eq!(res4 as f64, 32 as f64, epsilon = f64::EPSILON);
            }
        }
    };
}

make_test!(f32);
make_test!(f64);
