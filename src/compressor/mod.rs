pub trait ICompressor{
    fn compress(input: String) -> String;
    fn extract(input: String) -> String;
}

pub struct Compressor{}

impl ICompressor for Compressor{
    fn compress(input: String) -> String {
        return input;
    }

    fn extract(input: String) -> String {
        return input;
    }
}

#[cfg(test)]
mod tests_compressor {

    use crate::compressor::{Compressor, ICompressor};


    #[test]
    fn basic_test_compressor() {
        let input = "hello world";
        assert_eq!(input, Compressor::extract(Compressor::compress(String::from(input))));
    }
}
