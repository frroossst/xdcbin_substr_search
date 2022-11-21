#[cfg(test)]

mod test
    {
    use xdcbin_substr::helper_util::*;

    #[test]
    fn basic_test()
        {
        let mut str: &str = "1234"; 
        let mut num: u128 = convert_string_slice_to_u128(str);
        assert_eq!(1234, num);

        str = "0";
        num = convert_string_slice_to_u128(str);
        assert_eq!(0, num);

        str = "10";
        num = convert_string_slice_to_u128(str);
        assert_eq!(10, num);

        str = "1234567890";
        num = convert_string_slice_to_u128(str);
        assert_eq!(1234567890, num);

        str = "999";
        num = convert_string_slice_to_u128(str);
        assert_eq!(999, num);

        str = "328947";
        num = convert_string_slice_to_u128(str);
        assert_eq!(328947, num);

        str = "01";
        num = convert_string_slice_to_u128(str);
        assert_eq!(1, num);
        }
    }