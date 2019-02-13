use std::fmt;

#[derive(Debug)]
struct Structure(i32);

#[derive(Debug)]
struct Deep(Structure);

#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8
}

struct BetterStruct(i32);
impl fmt::Display for BetterStruct {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug)]
struct Complex {
    real: f64,
    img: f64,
}
impl fmt::Display for Complex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} + {}i", self.real, self.img)
    }
}

struct List(Vec<i32>);

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let vec = &self.0;

        write!(f, "[")?;
        for (count, v) in vec.iter().enumerate() {
            if count != 0 { write!(f, ", ")?; }
            write!(f, "{}: {}", count, v)?;
        }

        write!(f, "]")
    }
}


#[derive(Debug)]
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        write!(f, "RGB ({red} {green} {blue}) 0x:{red:02X}{green:02X}{blue:02X}",
            red = self.red, green = self.green, blue = self.blue)
    }
}

fn main() {
    println!("Hello world");
    println!("I'm a Rustacean");

    let pi = 3.141592;
    println!("Pi is roughly {:.3}", pi);

    println!("{:?}", Structure(3));
    println!("{:?}", Deep(Structure(2)));

    println!("Pretty print {:#?}", Structure(3));
    println!("Pretty print {:#?}", Deep(Structure(2)));

    let name = "Peter";
    let age = 24;
    let peter = Person{name, age};
    println!("Pretty {:#?}", peter);

    println!("Better struct {}", BetterStruct(5));

    let c = Complex{img: 3.3, real: 7.2};
    println!("Display: {}", c);
    println!("Debug {:?}", c);

    let v = List(vec![1, 2, 3]);
    println!("{}", v);

     for color in [
        Color { red: 128, green: 255, blue: 90 },
        Color { red: 0, green: 3, blue: 254 },
        Color { red: 0, green: 0, blue: 0 },
    ].iter() {
        println!("{}", *color);
    }
}
