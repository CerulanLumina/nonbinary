

pub struct DeserializerBuilder {

}

pub struct TypeBuilder<'a, Parent> {
    parent: &'a mut Parent
}

impl DeserializerBuilder {
    fn new() -> DeserializerBuilder {
        unimplemented!()
    }

    fn register_type<'a, T: Default + Copy + 'a, F: FnMut(TypeBuilder<'a, T>) -> TypeBuilder<'a, T>>(self, type_builder: F) -> Self {
        unimplemented!()
    }
}

impl<'a, Parent> TypeBuilder<'a, Parent> {
    fn new(parent: Parent) -> TypeBuilder<Parent> {
        unimplemented!()
    }

    fn register_field<X: Default + Copy + 'a, F: FnMut(&'a mut Parent) -> &'a mut X, G: Fn(X)>(self, accessor: F, a: G) -> Self {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use crate::deserializer::DeserializerBuilder;

    #[derive(Copy, Clone, Default)]
    struct Demo {
        pub val1: u64,
        pub val2: u32,
    }

    #[test]
    fn test_lifetime() {

        DeserializerBuilder::new()
            .register_type::<Demo, _>(|a| {
                a
                    .register_field(|a| &mut a.val1, |a|())
                    .register_field(|a| &mut a.val2, |a|())
            });



    }

    fn get_asd<'a>() -> &'a mut u32 {
        unimplemented!()
    }


    fn idea_pointer_move<T, U>() {
        let a = get_asd();
        let void = 0usize as *const u32;
        *a = unsafe { *void };
    }

}
