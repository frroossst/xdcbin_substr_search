pub struct StrToU128
    {
    slice: String,
    numbr: u128,
    }

impl StrToU128
    {
    pub fn new(slice: String) -> Self
        {
        StrToU128 { slice: slice, numbr: 0 }
        }

    pub fn display(&self)
        {
        println!("slice: {:?}; number: {:?}", self.slice, self.numbr);
        }
    }

pub fn convert_string_slice_to_u128(s: &str) -> u128
    {
    let stu = StrToU128::new(s.to_string().chars().rev().collect());

    let mut whole: u128 = 0;
    for (x, i) in stu.slice.chars().enumerate()
        {
        if !i.is_whitespace()
            {
            let pow10: u128 = 10_u128.pow(x.try_into().unwrap());
            let num: u128 = i.to_digit(10).unwrap().try_into().unwrap();
            whole += pow10 * num;
            }
        }
    whole
    }