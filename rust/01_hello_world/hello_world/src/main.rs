use std::fmt;


fn main() {
    cap1();
    cap1_2_1();
    cap1_2_2_1();
    cap1_2_3();
}

fn cap1(){
    println!("Hello, world!");

    // 1.2
    println!("--{}--", 42);
    println!("{0}{0}{2}{1}", "a",10,"*");
    println!(
        "{owner}.{table_name}.{column_name}",
        owner = "orion",
        table_name = "t_starts",
        column_name = "size"
    );
    println!("{0} = 0x{0:X} = 0b{0:b}",10);
    println!("{n:>w$}", n=42, w=5);
    println!("{n:>0w$}", n=42, w=5);
    println!("{n:<w$}", n=42, w=5);
    println!("{n:<0w$}", n=42, w=5);
    println!("{n:>05}", n=42);
    //println!("{0}{1}", 1);

    // 演習
    println!("Pi is rough ly {:.3}", 22.0/7.0);
}

// 1.2.1 デバッグとfmt::Display
#[derive(Debug)]
struct MinMax(i64, i64);

impl fmt::Display for MinMax {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.0, self.1)
    }
}

#[derive(Debug)]
struct Point2 {
    x: f64,
    y: f64,
}

impl fmt::Display for Point2 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "x: {}, y: {}", self.x, self.y)
    }
}


// 1.2.1 演習
#[derive(Debug)]
struct Complex {
    real: f64,
    imag: f64,
}

impl fmt::Display for Complex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} + {}i", self.real, self.imag)
    }
}

fn cap1_2_1(){
    #[derive(Debug)]
    struct Structure(i32);

    #[derive(Debug)]
    struct Deep(Structure);

    println!("<{:?}>", 12);
    println!("[{:?}]", Structure(3));
    println!("[{:?}]", Deep(Structure(7)));

    println!("{:?}", MinMax(10,20));
    println!("{}", MinMax(10,20));

    println!("{:?}", Point2 { x: 3.3, y: 7.2});
    println!("{}", Point2 { x: 3.3, y: 7.2});

    println!("{:?}", Complex { real: 3.3, imag: 7.2});
    println!("{}", Complex { real: 3.3, imag: 7.2});

}


// 1.2.2.1
struct List(Vec<i32>);

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let List(ref vec) = *self;
        try!(write!(f, "["));
        for(count, v) in vec.iter().enumerate(){
            if count != 0 {try!(write!(f, ", "));}
            try!(write!(f, "{}", v));
        }
        write!(f, "]")
    }
}

fn cap1_2_2_1(){
    let v1 = List(vec![1,2,3]);
    println!("{}", v1);
}

// 1.2.3 フォーマット
struct City {
    name: &'static str,
    // 経度
    lat: f32,
    // 緯度
    lon: f32,
}
impl fmt::Display for City {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let lat_c = if self.lat >= 0.0 {'N'} else {'S'};
        let lon_c = if self.lon >= 0.0 {'E'} else {'W'};
        write!(f,
            "{}: {:.3}°{} {:.3}°{}",
            self.name,
            self.lat.abs(), lat_c,
            self.lon.abs(), lon_c,
        )
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
        write!(f,
        "RGB ({0}, {1}, {2}) 0x{0:02X}{1:02X}{2:02X}",
         self.red, self.green, self.blue)
    }
}
fn cap1_2_3(){
    for city in [
        City {name: "Dublin", lat: 53.347778, lon: -6.259722},
        City {name: "Oslo", lat: 59.95, lon: 10.75},
        City {name: "Vancouver", lat: 49.25, lon: -123.1},
    ].iter() {
        println!("{}", *city);
    }

    let colors = [
        Color{red: 128, green: 255, blue: 90},
        Color{red: 0, green: 3, blue: 254},
        Color{red: 0, green: 0, blue: 0},
    ];
    for color in colors.iter() {
        println!("{:?}", *color);
    }
    for color in colors.iter() {
        println!("{}", *color);
    }
}
