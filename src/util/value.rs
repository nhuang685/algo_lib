use std::fmt::Debug;
use std::hash::Hash;

pub trait Value<T>:
    Copy + Clone + PartialEq + Eq + PartialOrd + Ord + Hash + Debug + Default
{
    fn val() -> T;
    fn set_val(v: T);
}

#[macro_export]
macro_rules! create_value {
    ($name: ident, $v: vis $vname: ident: $t: ty = $val: literal) => {
        #[allow(non_upper_case_globals)]
        static mut $vname: $t = $val;
        #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Default)]
        pub struct $name;
        impl $crate::util::value::Value<$t> for $name {
            fn val() -> $t {
                return unsafe { $vname };
            }
            fn set_val(v: $t) {
                unsafe {
                    $vname = v;
                }
            }
        }
    };
}
