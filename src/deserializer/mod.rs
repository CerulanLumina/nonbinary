// 👻
use core::marker::PhantomData;

pub trait VecLike<T>: Extend<T> + AsRef<[T]> + Default {}
impl<V, T: Extend<V> + AsRef<[V]> + Default> VecLike<V> for T {}

pub struct DeserializerBuilder<T, V>
where
    T: VecLike<DataType<V>>,
    V: VecLike<DataField>,
{
    _phantom: PhantomData<V>,
    inner: T,
}

impl<T, V> DeserializerBuilder<T, V>
where
    T: VecLike<DataType<V>>,
    V: VecLike<DataField>,
{
    pub fn new() -> DeserializerBuilder<T, V> {
        DeserializerBuilder {
            _phantom: PhantomData,
            inner: T::default(),
        }
    }

    pub fn register_type(&mut self, data_type: DataType<V>) -> usize {
        let id = self.inner.as_ref().len();
        self.inner.extend([data_type]);
        id
    }

    pub fn build(self) -> Deserializer<T, V> {
        Deserializer::from_veclike(self.inner)
    }
}

pub struct Deserializer<T, V>
where
    T: VecLike<DataType<V>>,
    V: VecLike<DataField>,
{
    _phantom: PhantomData<V>,
    inner: T,
}

impl<T, V> Deserializer<T, V>
where
    T: VecLike<DataType<V>>,
    V: VecLike<DataField>,
{
    pub(crate) fn from_veclike(veclike: T) -> Deserializer<T, V> {
        assert!(
            veclike.as_ref().len() > 0,
            "Deserializer needs some types registered"
        );
        Deserializer {
            _phantom: PhantomData,
            inner: veclike,
        }
    }

    pub fn deserialize<D, Struct>(&self, from: D, out: &mut Struct) where D: AsRef<[u8]> {
        let root = &self.inner.as_ref()[0];
        root.deserialize(from, out);
    }
}

pub struct DataTypeBuilder<T, Struct>
where
    T: VecLike<DataField>,
    Struct:,
{
    structure: Struct,
    inner: T,
}

impl<'a, T, Struct> DataTypeBuilder<T, Struct>
where
    T: VecLike<DataField>,
    Struct: 'a + Copy,
{
    pub fn new(default_struct: Struct) -> DataTypeBuilder<T, Struct> {
        DataTypeBuilder {
            structure: default_struct,
            inner: T::default(),
        }
    }

    pub fn register_field<Return, Accessor>(&'a mut self, data_offset: usize, accessor: Accessor)
    where
        Return: 'a + Copy + Deserializable,
        Accessor: Fn(&'a Struct) -> &'a Return,
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
        impl FieldType {
            pub fn copy(&self, from_ptr_usize: usize, to_ptr_usize: usize) {
                match self {
                    $(
                    FieldType::$i => {
                        let from = from_ptr_usize as *mut $t;
                        let to = to_ptr_usize as *mut $t;
                        unsafe { *to = *from; }
                    },
                    )*
                }
            }
        }
    };
}

basic_impl!(
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64)
);

pub struct DataType<T>
where
    T: VecLike<DataField>,
{
    inner: T,
}

impl<T> DataType<T> where T: VecLike<DataField> {
    pub fn deserialize<D: AsRef<[u8]>, Struct>(&self, data: D, out: &mut Struct) {
        for x in self.inner.as_ref() {
            x.read_into(&data, out);
        }
    }
}

pub struct DataField {
    field_type: FieldType,
    data_offset: usize,
    ptr_offset: usize,
}

impl DataField {
    pub fn read_into<D: AsRef<[u8]>, Struct>(&self, data: D, out: &mut Struct) {
        let ptr_out = out as *mut _ as usize + self.ptr_offset;
        let ptr_in = &data.as_ref()[0] as *const _ as usize + self.data_offset;
        self.field_type.copy(ptr_in, ptr_out);
    }
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
}

#[cfg(test)]
mod builder_tests {
    use crate::deserializer::{
        DataField, DataType, DataTypeBuilder, DeserializerBuilder, FieldType, VecLike,
    };
    use arrayvec::ArrayVec;
    use std::prelude::rust_2021::*;

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

    #[derive(Default, Copy, Clone, Eq, PartialEq)]
    struct StructA {
        a: u64,
        b: u32,
        c: u16,
        d: u8,
        e: u64,
    }

    #[test]
    fn try_deser() {



        let mut owo = DeserializerBuilder::new();
        DataTypeBuilder::new(StructA::default())
            .register_field(0x0, |a| &a.a);
        owo.register_type()
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

        let test_verify = MyStruct {
            a: 32,
            b: 12,
            c: 64,
            d: 8,
        };
        let base_addr = refptr_usize!(&test_verify);

        assert_eq!(dtb.inner[0].field_type, FieldType::U32);
        assert_eq!(dtb.inner[0].data_offset, 0x0);
        assert_eq!(
            dtb.inner[0].ptr_offset + base_addr,
            refptr_usize!(&test_verify.a)
        );

        assert_eq!(dtb.inner[1].field_type, FieldType::U64);
        assert_eq!(dtb.inner[1].data_offset, 0x4);
        assert_eq!(
            dtb.inner[1].ptr_offset + base_addr,
            refptr_usize!(&test_verify.b)
        );

        assert_eq!(dtb.inner[2].field_type, FieldType::U8);
        assert_eq!(dtb.inner[2].data_offset, 0xc);
        assert_eq!(
            dtb.inner[2].ptr_offset + base_addr,
            refptr_usize!(&test_verify.c)
        );

        assert_eq!(dtb.inner[3].field_type, FieldType::U16);
        assert_eq!(dtb.inner[3].data_offset, 0xd);
        assert_eq!(
            dtb.inner[3].ptr_offset + base_addr,
            refptr_usize!(&test_verify.d)
        );

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
