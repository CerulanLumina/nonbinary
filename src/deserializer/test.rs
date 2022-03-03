struct NBField {

}

pub struct DeserializerBuilder<D: DeserializerImpl> {
    inner: D::BuilderImpl
}

impl<D: DeserializerImpl> DeserializerBuilder<D> {
    pub fn new(inner: D::BuilderImpl) -> DeserializerBuilder<D> {
        DeserializerBuilder::<D> { inner }
    }
}

pub struct Deserializer<D: DeserializerImpl> {
    inner: D
}

impl<D: DeserializerImpl> Deserializer<D> {
    fn builder() -> D::BuilderImpl {
        D::BuilderImpl::new()
    }
}

pub trait DeserializerImpl: Sized {
    type TypeImpl: NBTypeImpl;
    type BuilderImpl: DeserializerBuilderImpl<Self>;
    type TypeBuilderImpl<'a, T: 'a + Copy>: TypeBuilderImpl<'a, T>;
    fn builder<D: DeserializerImpl<BuilderImpl = T>, T: DeserializerBuilderImpl<D>>() -> DeserializerBuilder<D> {
        DeserializerBuilder::<D>::new(T::new())
    }
    fn types(&self) -> &[NBType<Self::TypeImpl>];
}

struct NBType<T: NBTypeImpl> {
    inner: T
}

trait DeserializerBuilderImpl<DI: DeserializerImpl>: Sized {
    fn new() -> Self;
    fn register_type<'a, TypeBuilderFunc, Struct>(self, key: usize, inner: Struct, type_builder: TypeBuilderFunc) -> Self
        where TypeBuilderFunc: FnOnce(DI::TypeBuilderImpl<'a, Struct>) -> DI::TypeBuilderImpl<'a, Struct>, Struct: Copy + 'a;
}

trait NBTypeImpl: Sized {
    fn fields(&self) -> &[NBField];
}

trait TypeBuilderImpl<'a, Struct: 'a + Copy>: Sized {
    fn register_field<Value: 'a, AccessFunc>(self, data_offset: usize, accessor: AccessFunc) -> Self
        where Value: Deserializable,
            AccessFunc: FnOnce(&'a mut Struct) -> &'a mut Value;
}

enum DeserializeFormat {
    U8,
    I8,
    U16,
    I16,
    U32,
    I32,
    U64,
    I64,
}

trait Deserializable: Sized + Copy {
    fn get_format() -> DeserializeFormat;
}

macro_rules! basic_impl {
    ($($i:tt($t:ty)),*) => {
        $(
        impl Deserializable for $t {
            fn get_format() -> DeserializeFormat { DeserializeFormat::$i }
        }
        )*
    };
}

basic_impl!(U8(u8), U16(u16), U32(u32), U64(u64), I8(i8), I16(i16), I32(i32), I64(i64));

mod impls_alloc_test {
    use crate::deserializer::test::*;

    pub struct AllocDeserializerBuilder {

    }

    pub struct AllocDeserializer {

    }

    pub struct AllocType {

    }

    pub struct AllocTypeBuilder<Struct: Copy> {
        default: Struct
    }

    impl<'a, T: 'a + Copy> TypeBuilderImpl<'a, T> for AllocTypeBuilder<T> {
        fn register_field<Value: 'a, AccessFunc>(self, data_offset: usize, accessor: AccessFunc) -> Self where Value: Deserializable, AccessFunc: FnOnce(&'a mut T) -> &'a mut Value {
            todo!()
        }
    }

    impl DeserializerImpl for AllocDeserializer {
        type TypeImpl = AllocType;
        type BuilderImpl = AllocDeserializerBuilder;
        type TypeBuilderImpl<'a, T: 'a + Copy> = AllocTypeBuilder<T>;

        fn types(&self) -> &[NBType<Self::TypeImpl>] {
            todo!()
        }
    }

    impl DeserializerBuilderImpl<AllocDeserializer> for AllocDeserializerBuilder {
        fn new() -> Self {
            todo!()
        }

        fn register_type<'a, TypeBuilderFunc, Struct>(self, key: usize, inner: Struct, type_builder: TypeBuilderFunc) -> Self where TypeBuilderFunc: FnOnce(AllocTypeBuilder<Struct>) -> AllocTypeBuilder<Struct>, Struct: Copy + 'a {
            todo!()
        }
    }

    impl NBTypeImpl for AllocType {
        fn fields(&self) -> &[NBField] {
            todo!()
        }
    }

    #[cfg(test)]
    mod tests {
        use crate::deserializer::test::{Deserializer, DeserializerBuilderImpl, TypeBuilderImpl};
        use crate::deserializer::test::impls_alloc_test::AllocDeserializer;

        struct Des {
            pub nyaa: u64,
            pub owo: u32,
        }

        #[test]
        fn lol() {

            let builder = Deserializer::<AllocDeserializer>::builder();
            builder.register_type(2, Des{nyaa: 0, owo: 0}, |a| {
                a
                    .register_field(0x0, |a| &mut a.owo)
                    .register_field(0x4, |a| &mut a.nyaa)
            });
        }
    }


}