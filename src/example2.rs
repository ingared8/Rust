pub fn string_test() {
    let s = String::from("Shree");
     //heap_example(s);
    heap_example_borrow(&s);
}

pub fn heap_example(input: String) {
    let mystr =  input;
    //let _otherstr = mystr;
    println!("{}", mystr);
}

fn heap_example_borrow(input: &String) {
    let mystr =  input;
    let _otherstr = mystr;
    println!("{}", mystr);
}

