#[cfg(test)]
mod tests {
    use arrow::datatypes::ToByteSlice;

    #[test]
    fn it_works() {
        let a = 255_u32;

        let c = a.to_be_bytes();
        let d = a.to_le_bytes();

        let s = a.to_byte_slice();

        assert_eq!(c, s);
    }
}
