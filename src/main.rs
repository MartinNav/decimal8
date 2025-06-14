use decimal8::*;
fn main() {
        for i in (0..95).step_by(5) {
            println!("3.{i} == {}",Decimal8::new(3, i));
        }
}
