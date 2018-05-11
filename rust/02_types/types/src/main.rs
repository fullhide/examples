use std::fmt;

fn main() {
    cap2_2();
    cap2_3();
}

// 2.2
fn reverse(pair: (i32, bool)) -> (bool, i32) {
    let (i, b) = pair;
    (b, i)
}

#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);

impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({} {})\n({} {})", self.0, self.1, self.2, self.3)
    }
}
fn transpose(m :Matrix) -> Matrix {
    Matrix(m.0, m.2, m.1, m.3)
}
fn cap2_2(){
    let long_tuple = (1u8, 2u16, 3u32, 4u64, 'a', true);
    println!("first : {}", long_tuple.0);
    println!("second: {}", long_tuple.1);
    println!("{:?}", long_tuple);

    let pair = (100, true);
    println!("{:?}", pair);
    println!("{:?}", reverse(pair));

    let tuple = (1, "hello", 4, 5, true);
    let (a, b, c, d, _) = tuple;
    println!("{:?},{:?},{:?},{:?}", a,b,c,d);

    let matrix = Matrix(1.1, 1.2, 2.1, 2.2);
    println!("{:?}", matrix);
    println!("{}", matrix);
    println!("{}", transpose(matrix));
}

// 2.3 array , slice
fn cap2_3(){
    let xs: [i32; 5] = [1,2,3,4,5];
    let ys: [i32; 500] =[0; 500];
    println!("xs[0] = {}", xs[0]);
    println!("xs[1] = {}", xs[1]);
}