// pub mod nonalloc;
// mod test;

// 👻
use core::marker::PhantomData;


pub trait VecLike<T>: Extend<T> + AsRef<[T]> + Default {}
impl<V, T: Extend<V> + AsRef<[V]> + Default> VecLike<V> for T {}


pub struct DeserializerBuilder<T, V> where T: VecLike<DataType<V>>, V: VecLike<DataField> {
    _phantom: PhantomData<V>,
    inner: T
}

impl<T, V> DeserializerBuilder<T, V> where T: VecLike<DataType<V>>, V: VecLike<DataField> {
    pub fn new() -> DeserializerBuilder<T, V> {
        DeserializerBuilder { _phantom: PhantomData, inner: T::default() }
    }

    pub fn register_type(mut self, data_type: DataType<V>) -> Self {
        self.inner.extend([data_type]);
        self
    }
}

pub struct DataTypeBuilder<T, Struct> where T: VecLike<DataField>, Struct: {
    structure: Struct,
    inner: T
}

impl<'a, T, Struct> DataTypeBuilder<T, Struct> where T: VecLike<DataField>, Struct: 'a + Copy {
    pub fn new(default_struct: Struct) -> DataTypeBuilder<T, Struct> {
        DataTypeBuilder { structure: default_struct, inner: T::default() }
    }

    pub fn register_field<Return, Accessor>(&'a mut self, data_offset: usize, accessor: Accessor)
        where Return: 'a + Copy + Deserializable, Accessor: Fn(&'a Struct) -> &'a Return
    {
        let struct_ref = &self.structure;
        let struct_ptr = struct_ref as *const Struct as usize;
        let reference = accessor(struct_ref);
        let reference_ptr = reference as *const Return as usize;

        let new_field = DataField {
            field_type: Return::field_type(),
            data_offset,
            ptr_offset: reference_ptr - struct_ptr,
        };

        self.inner.extend([new_field]);
    }
}

pub trait Deserializable: Sized + Copy {
    fn field_type() -> FieldType;
}

macro_rules! basic_impl {
    ($($i:tt($t:ty)),*) => {
        $(
        impl Deserializable for $t {
            fn field_type() -> FieldType { FieldType::$i }
        }
        )*
    };
}

basic_impl!(U8(u8), U16(u16), U32(u32), U64(u64), I8(i8), I16(i16), I32(i32), I64(i64));

pub struct DataType<T> where T: VecLike<DataField> {
    inner: T,
}

pub struct DataField {
    field_type: FieldType,
    data_offset: usize,
    ptr_offset: usize,
}

#[cfg_attr(test, derive(Debug))]
#[derive(Eq, PartialEq)]
pub enum FieldType {
    U8,
    I8,
    U16,
    I16,
    U32,
    I32,
    U64,
    I64,
    DataType(usize),
}

#[cfg(test)]
mod builder_tests {
    use crate::deserializer::{DataField, DataType, DataTypeBuilder, DeserializerBuilder, FieldType, VecLike};
    use std::prelude::rust_2021::*;
    use arrayvec::ArrayVec;

    macro_rules! refptr_usize {
        ($e:expr) => { $e as *const _ as usize };
    }

    #[test]
    fn build_simple_data_type_alloc() {
        build_simple_data_type::<Vec<_>>();
    }

    #[test]
    fn build_simple_data_type_nonalloc() {
        build_simple_data_type::<ArrayVec<_, 4>>();
    }

    fn build_simple_data_type<T: VecLike<DataField>>() {

        #[derive(Copy, Clone, Default)]
        #[allow(unused)]
        struct MyStruct {
            a: u32,
            b: u64,
            c: u8,
            d: u16,
        }

        let mut dtb = DataTypeBuilder::<Vec<_>, _>::new(MyStruct::default());
        dtb.register_field(0x0, |a| &a.a);
        dtb.register_field(0x4, |a| &a.b);
        dtb.register_field(0xc, |a| &a.c);
        dtb.register_field(0xd, |a| &a.d);

        let test_verify = MyStruct { a: 32, b:12, c:64, d:8 };
        let base_addr = refptr_usize!(&test_verify);

        assert_eq!(dtb.inner[0].field_type, FieldType::U32);
        assert_eq!(dtb.inner[0].data_offset, 0x0);
        assert_eq!(dtb.inner[0].ptr_offset + base_addr, refptr_usize!(&test_verify.a));

        assert_eq!(dtb.inner[1].field_type, FieldType::U64);
        assert_eq!(dtb.inner[1].data_offset, 0x4);
        assert_eq!(dtb.inner[1].ptr_offset + base_addr, refptr_usize!(&test_verify.b));

        assert_eq!(dtb.inner[2].field_type, FieldType::U8);
        assert_eq!(dtb.inner[2].data_offset, 0xc);
        assert_eq!(dtb.inner[2].ptr_offset + base_addr, refptr_usize!(&test_verify.c));

        assert_eq!(dtb.inner[3].field_type, FieldType::U16);
        assert_eq!(dtb.inner[3].data_offset, 0xd);
        assert_eq!(dtb.inner[3].ptr_offset + base_addr, refptr_usize!(&test_verify.d));

        let ptr_a = (base_addr + dtb.inner[0].ptr_offset) as *const u32;
        let ptr_b = (base_addr + dtb.inner[1].ptr_offset) as *const u64;
        let ptr_c = (base_addr + dtb.inner[2].ptr_offset) as *const u8;
        let ptr_d = (base_addr + dtb.inner[3].ptr_offset) as *const u16;

        assert_eq!(ptr_a, (&test_verify.a) as *const u32);
        assert_eq!(ptr_b, (&test_verify.b) as *const u64);
        assert_eq!(ptr_c, (&test_verify.c) as *const u8);
        assert_eq!(ptr_d, (&test_verify.d) as *const u16);

        unsafe {
            assert_eq!(*ptr_a, test_verify.a);
            assert_eq!(*ptr_b, test_verify.b);
            assert_eq!(*ptr_c, test_verify.c);
            assert_eq!(*ptr_d, test_verify.d);
        }

    }

}
