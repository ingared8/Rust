extern crate term;

pub fn color_write() {
    let mut t = term::stdout().unwrap();

    t.fg(term::color::GREEN).unwrap();
    write!(t, "Hello ").unwrap();

    t.fg(term::color::RED).unwrap();
    write!(t, " World !").unwrap();

    t.reset().unwrap();
}

