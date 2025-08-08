pub mod my_error;
pub mod my_iterator_ext;

use std::{any::TypeId, ffi::os_str::Display};

pub use self::{my_error::MyError, my_iterator_ext::MyIteratorExt};

/// ```compile_fail
/// // we were able to impl private::Sealed for types in my_iterator_ext,
/// // but we are not able to do it in downstream crates
/// impl my_iterator_ext::private::Sealed for DownStreamType {} // shouldn't compile!
/// ```

struct DownStreamType {}
impl Iterator for DownStreamType {
    type Item = ();
    fn next(&mut self) -> Option<Self::Item> {
        None
    }
}

// compile_fail, MyIteratorExt is Sealed
// impl MyIteratorExt for DownStreamType {
//     fn format(self, sep: &str) -> Format<Self>
//     where
//         Self: Sized,
//     {
//         format::new_format_default(self, sep)
//     }

//     fn format_with<F>(self, sep: &str, format: F) -> FormatWith<Self, F>
//     where
//         Self: Sized,
//         F: FnMut(Self::Item, &mut dyn FnMut(&dyn std::fmt::Display) -> std::fmt::Result) -> std::fmt::Result,
//     {
//         format::new_format(self, sep, format)
//     }
// }

// compile_fail, module format is private
// fn use_sealed(
//     value: impl my_iterator_ext::MyIteratorExt,
// ) -> my_iterator_ext::format::Format<'_, impl my_iterator_ext::MyIteratorExt> {
//     value.format("separator")
// }

#[derive(Debug)]
pub struct DownStreamType2 {}
impl std::fmt::Display for DownStreamType2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl MyError for DownStreamType2 {
    fn source(&self) -> Option<&(dyn MyError + 'static)> {
        None
    }

    // Can not implement method because of type private::Token
    // fn type_id(&self, _: my_error::private::Token) -> std::any::TypeId
    // where
    //     Self: 'static,
    // {
    //     TypeId::of::<Self>()
    // }
}
