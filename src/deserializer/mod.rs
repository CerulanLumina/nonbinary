// pub mod nonalloc;
mod test;

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


