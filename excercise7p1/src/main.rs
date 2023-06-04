fn multiply(x: i16, y: i16) -> i16 {
    x * y
}

fn main() {
    let x: bool = true;
    let y: i16 = 1000;
    let z = multiply(x.into(), y);

    println!("{x} * {y} = {z}");
}