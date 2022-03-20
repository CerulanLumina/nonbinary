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

// #[cfg(feature = "std")]
// pub mod alloc;
//
// pub struct DeserializerBuilder<D: DeserializerBuilderImpl> {
//     inner: D,
// }
//
// trait DeserializerBuilderImpl: Sized {
//     type DeserializerImpl: DeserializerImpl;
//     type NBTypeBuilder: NBTypeBuilderImpl;
//     type NBType: NBTypeImpl;
//     fn new() -> Self;
//     fn register_type(mut self, key: usize, new_type: NBType<Self::NBType>) -> Self;
//     fn register_type_builder<F>(mut self, key: usize, builder: F) -> Self
//         where F: FnOnce(Self::NBTypeBuilder) -> Self::NBTypeBuilder + 'static {
//         let new_type = builder(Self::NBTypeBuilder::new()).build();
//         self.register_type(key, new_type);
//         self
//     }
//     fn set_root_type(mut self, root: usize);
//     fn build(self) -> NBDeserializer<Self::DeserializerImpl>;
// }
//
// trait NBTypeBuilderImpl: Sized {
//     type TypeImpl: NBTypeImpl;
//     fn new() -> Self;
//     fn register_field(self) -> Self;
//     fn build(self) -> NBType<Self::TypeImpl>;
// }
//
// struct NBDeserializer<D: DeserializerImpl> {
//     inner: D
// }
//
// impl<D: DeserializerImpl> NBDeserializer<D> {
//
// }
//
// trait DeserializerImpl: Sized {
//     fn root_type(&self) -> usize;
//     fn types(&self) -> &[NBType<>]
// }
//
//
//
// struct NBType<I: NBTypeImpl> {
//     inner: I
// }
//
// struct NBField {
//     field_type: FieldType,
//     struct_offset: usize,
//     data_offset: usize,
// }
//
// pub enum FieldType {
//     I8,
//     U8,
//     I16,
//     U16,
//     I32,
//     U32,
// }
//
// trait NBTypeImpl: Sized {
//     fn fields(&self) -> &[NBField];
// }


