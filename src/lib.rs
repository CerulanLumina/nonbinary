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
extern crate std;
#[cfg(test)]
extern crate arrayvec;

#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::io::Read;

    #[allow(unused)]
    fn get_data_bin() -> [u8; 1024] {
        let mut b = [0u8; 1024];

        let mut f = File::open("data/test.bin").expect("opening file");
        f.read_exact(&mut b).expect("reading file");
        b
    }

}
