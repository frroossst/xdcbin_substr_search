struct str_to_u128
    {
    slice: String,
    numbr: u128,
    }

impl str_to_u128
    {
    fn new(slice: String) -> Self
        {
        str_to_u128 { slice: slice, numbr: 0 }
        }

    fn display(&self)
        {
        print!("slice: {:?}; number: {:?}", self.slice, self.numbr);
        }
    }

pub fn convert_string_slice_to_u128(s: &str) -> u128
    {
    let stu = str_to_u128::new(s.to_string().chars().rev().collect());

    stu.display();

    return 100;
    }