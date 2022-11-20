#[cfg(test)]

mod test    
    {
    use xdcbin_substr::def_word::WordStruct;
    use std::cmp::Ordering;

    #[test]    
    fn word_struct_eq()
        {
        let ws = WordStruct::new("lorem");
        let ws_cmp_eq = WordStruct::new("lorem");

        assert_eq!(ws.cmp(&ws_cmp_eq), Ordering::Equal);
        assert_eq!(ws_cmp_eq.cmp(&ws), Ordering::Equal);
        }

    #[test]    
    fn word_struct_less()
        {
        let ws = WordStruct::new("lorem");
        let ws_cmp_eq = WordStruct::new("ipsum");

        assert_eq!(ws_cmp_eq.cmp(&ws), Ordering::Less);
        }
        
    #[test]    
    fn word_struct_greater()
        {
        let ws = WordStruct::new("lorem");
        let ws_cmp_eq = WordStruct::new("ipsum");

        assert_eq!(ws.cmp(&ws_cmp_eq), Ordering::Greater);
        }
        
    }


