// Example Hello- world 

extern crate term;

fn main() {

    string_test();
    stack_test();
    color_write();
}

fn color_write() {
    let mut t = term::stdout().unwrap();
    t.fg(term::color::GREEN).unwrap();
    write!(t,"Hello ").unwrap();

    t.fg(term::color::RED).unwrap();
    write!(t," World !").unwrap();

    t.reset().unwrap();

}

fn string_test() {
    let s = String::from("Shree");
     //heap_example(s);
    heap_example_borrow(&s);
}


fn heap_example(input: String) {
    let mystr =  input;
    //let _otherstr = mystr;
    println!("{}", mystr);
}

fn heap_example_borrow(input: &String) {
    let mystr =  input;
    let _otherstr = mystr;
    println!("{}", mystr);
}

fn stack_test(){
    let i = 12121993;
    stack_example(i);
}

fn stack_example(input: i32){
    let x = input;
    let _y = x;
    println!("{}", x);
}


