#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    InvalidInputBase,
    InvalidOutputBase,
    InvalidDigit(u32),
}

///
/// Convert a number between two bases.
///
/// A number is any slice of digits.
/// A digit is any unsigned integer (e.g. u8, u16, u32, u64, or usize).
/// Bases are specified as unsigned integers.
///
/// Return the corresponding Error enum if the conversion is impossible.
///
///
/// You are allowed to change the function signature as long as all test still pass.
///
///
/// Example:
/// Input
///   number: &[4, 2]
///   from_base: 10
///   to_base: 2
/// Result
///   Ok(vec![1, 0, 1, 0, 1, 0])
///
/// The example corresponds to converting the number 42 from decimal
/// which is equivalent to 101010 in binary.
///
///
/// Notes:
///  * The empty slice ( "[]" ) is equal to the number 0.
///  * Never output leading 0 digits, unless the input number is 0, in which the output must be `[0]`.
///    However, your function must be able to process input with leading 0 digits.
///

pub fn convert(number: &[u32], from_base: u32, to_base: u32) -> Result<Vec<u32>, Error> {
    if number == [] {
        return Ok(vec![0]);
    }
    if from_base<2{return Err(Error::InvalidInputBase);}
    if to_base<2{return Err(Error::InvalidOutputBase);}
    let mut num_in_base_10 = 0;
    for pos in (0..number.len()) {
        if number[pos] >= from_base{
            return Err(Error::InvalidDigit(number[pos]));
        }
        num_in_base_10 += from_base.pow(pos as u32) * number[pos]
    }
    println!("{num_in_base_10}");
    let mut biggest_power = 0;
    loop {
        if to_base.pow(biggest_power) < num_in_base_10{
            biggest_power +=1;
        } else {
            break;
        }
    }
    let mut number_in_new_base :Vec<u32>= vec![];

    for i in (0..biggest_power).rev(){
        number_in_new_base.push(num_in_base_10/to_base.pow(i));
    }
    Ok(number_in_new_base)
}
