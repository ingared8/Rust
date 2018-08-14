// Example Hello- world 

extern crate term;
mod example1;
mod example2;
mod example3;

fn main() {
   let mut i = 1;
   println!("Start of example {}", i);
   example1::color_write();
   println!("End of example  {}", i);

    i += 1;
    println!("Start of example {}", i);
    example2::string_test();
    println!("End of example {}", i);

    i+= 1;
    println!("Start of example {}", i);
    example3::stack_test();
    println!("End of example {}", i);
}





