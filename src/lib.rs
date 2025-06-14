// Crate to represent 8 bit decimal numbers
// bits will be as follows |sign(1)|integer(4)|decimal(3)

use std::fmt;

#[derive(Clone,Copy)]
pub struct Decimal8{
    val:i8,
}
impl Decimal8 {
    pub fn new(integer:i8,decimal:u8)->Self{
        let mut d=decimal;
        if d>99 {
            d= d/10;
        }
        if d!=0 {
            d = 100/d;
        }
        let val  = (-128&integer)|((integer<<3)&0b01111000)|(((d>>3)&0b00000111)as i8);

        Decimal8{val}
    }
}
impl fmt::Display for Decimal8 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let integer = (self.val&0b01111000)>>3|(self.val&-128);
        let mut decimal:u16 = {
            let v = (self.val&0b00000111)as u16;
            if v==0 {
               v 
            }else {
                100/v
            }
        };
        if decimal%10==0 {
            decimal=decimal/10;
        }
        write!(f,"{integer}.{decimal}")
    }
}
impl std::ops::Add for Decimal8 {
    type Output = Decimal8;
    fn add(self, rhs: Self) -> Self::Output {
        Decimal8{val: self.val+ rhs.val}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!("3.5",format!("{}",Decimal8::new(3, 5)));
    }
}
