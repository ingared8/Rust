// Example Hello- world 

extern crate term;

fn main() {
    let mut t = term::stdout().unwrap();
    t.fg(term::color::GREEN).unwrap();
    write!(t,"Hello ").unwrap();

    t.fg(term::color::RED).unwrap();
    write!(t," World !").unwrap();

    t.reset().unwrap();

    stringTest()
}

fn stringTest() {
    let s = String::from("Test");
     heap_example(s);
}


fn heap_example(input: String) {
    let mystr =  input;
    let _otherstr = mystr;
    println!("{}", mystr)
}
