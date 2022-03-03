//! You can pass `NB_ARRAY_SIZE` as an environment variable at compile time to override
//! the size of the stack-backed arrays.

use arrayvec::ArrayVec;
use crate::deserializer::{NBDeserializer, NBField, NBType};

mod nonalloc_const_def;

pub struct NonAllocDeserializer {
    types: ArrayVec<NBType<NBTypeImpl>, {nonalloc_const_def::max_array_size()}>
}

pub struct NonAllocDeserializerBuilder {
    types: ArrayVec<NBType<NBTypeImpl>, {nonalloc_const_def::max_array_size()}>
}

pub struct NBTypeImpl {
    fields: ArrayVec<NBField, {nonalloc_const_def::max_array_size()}>
}

impl crate::deserializer::DeserializerBuilderImpl for NonAllocDeserializerBuilder {
    type DeserializerImpl = NonAllocDeserializer;
    type NBTypeBuilder = ;
    type NBType = ();

    fn new() -> Self {
        todo!()
    }

    fn register_type(self, key: usize, new_type: NBType<Self::NBType>) -> Self {
        todo!()
    }

    fn set_root_type(self, root: usize) {
        todo!()
    }

    fn build(self) -> NBDeserializer<Self::DeserializerImpl> {
        todo!()
    }
}



// use core::ffi::c_void;

// pub struct DeserializerBuilder {}
//
// pub struct TypeBuilder<'a, Parent> {
//     parent: &'a mut Parent,
// }
//

//
// pub struct Field<'a, Parent> {
//     typ: FieldType,
//     accessor: *mut u8,
//     offset: usize,
// }
//
// impl DeserializerBuilder {
//     fn new() -> DeserializerBuilder {
//         unimplemented!()
//     }
//
//     fn register_type<'a, T, F>(self, type_builder: F) -> Self
//     where
//         T: Default + Copy + 'a,
//         F: FnOnce(TypeBuilder<'a, T>) -> TypeBuilder<'a, T>,
//         F: 'static,
//     {
//         unimplemented!()
//     }
// }
//
// impl<'a, Parent> TypeBuilder<'a, Parent> {
//     fn new(parent: Parent) -> TypeBuilder<'a, Parent> {
//         unimplemented!()
//     }
//
//     fn register_field<X, F>(self, accessor: F) -> Self
//     where
//         X: Default + Copy + 'a,
//         F: FnOnce(&'a mut Parent) -> &'a mut X,
//         F: 'static,
//     {
//         unimplemented!()
//     }
// }
//
// #[cfg(test)]
// mod tests {
//     use crate::deserializer::DeserializerBuilder;
//
//     #[derive(Copy, Clone, Default)]
//     struct Demo {
//         pub val1: u64,
//         pub val2: u64,
//     }
//
//     #[test]
//     fn test_lifetime() {
//         DeserializerBuilder::new().register_type::<Demo, _>(|a| {
//             a.register_field(|a| &mut a.val1)
//                 .register_field(|a| &mut a.val2)
//         });
//     }
//
//     fn get_asd<'a>() -> &'a mut u32 {
//         unimplemented!()
//     }
//
//     fn idea_pointer_move<T, U>() {
//         let a = get_asd();
//         let void = 0usize as *const u32;
//         *a = unsafe { *void };
//     }
// }
