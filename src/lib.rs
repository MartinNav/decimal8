// Crate to represent 8 bit decimal numbers
// bits will be as follows |sign(1)|whole numbers(4)|decimals(3)

#[derive(Clone,Copy)]
pub struct Decimal8{
    val:i8,
}
impl std::ops::Add for Decimal8 {
    fn add(self, rhs: Self) -> Self::Output {
               
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
