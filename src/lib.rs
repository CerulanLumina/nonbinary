#![cfg_attr(not(any(test, feature = "std")), no_std)]
// mod deserializer;

pub mod deserializer;

pub fn add(a: u32, b: u32) -> u32 {
    let sum = a + b;
    if sum == 42 {
        panic!("WHAT IS THE QUESTION THOUGH");
    }
    sum
}



#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::io::Read;
    use std::ops::Deref;

    struct Template<'a> {
        _data: &'a [u8],
        pub coolnum: &'a u32,
    }

    impl<'a> Template<'a> {
        fn create(data: &'a [u8]) -> Template<'a> {
            let owo = (data.as_ptr() as usize + 0x20) as *const u32;
            let coolnum = unsafe { &*owo };
            Template {
                _data: data,
                coolnum,
            }
        }
    }

    struct Test {
        pub v1: u64,
        pub v2: u64,
    }

    #[test]
    fn test_asdf() {
        let mut v = Test { v1: 0, v2: 0 };

        let data = get_data_bin();

        // Deserialzer.register_type("Test", Test::default())
        //
    }

    fn get_data_bin() -> [u8; 1024] {
        let mut b = [0u8; 1024];

        let mut f = File::open("data/test.bin").expect("opening file");
        f.read_exact(&mut b).expect("reading file");
        b
    }

    #[test]
    fn idea_proof_of_concept() {
        let b = get_data_bin();

        let templ = Template::create(&b);

        assert_eq!(*templ.coolnum, 70499491);
    }
}
