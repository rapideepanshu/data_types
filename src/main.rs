fn main() {
    // Scaler Data types
    // 1 ) Integer
    let x: i8 = 10;

    println!("{}", x);

    let y: u8 = 19;

    println!("{}", y);

    let _z: u8 = 10;

    let decimal = 02_55;
    let hex = 0xff;
    let octal = 0o377;
    let binary = 0b1111_1111;

    println!("{}", decimal);
    println!("{}", hex);
    println!("{}", octal);
    println!("{}", binary);

    let byte = b'A';

    println!("{}", byte);

    // 2) Floating Points

    let a = 2.0;
    let b: f32 = 3.0;

    println!("{}", a);
    println!("{}", b);

    // 3) Boolean

    let c = true;
    let d: bool = false;

    println!("{}", c);
    println!("{}", d);

    // 4) Character

    let e = 'e';
    println!("{}", e);
    // Arithmetic operators ,-,*,/,%

    let s = 10;
    let t = 4;
    let u = s % t;

    println!("{}", u);

    // Compound Types

    // 1) Tuple- Assign multiple values of different types
    let tup = (2, "hi", true);
    println!("{}", tup.1);

    let (i, j, k) = tup;
    println!("{}", i);
    println!("{}", j);
    println!("{}", k);

    // 2) Array- Assign multiple values of same type

    let array = [1, 2, 3];

    println!("{}", array[0]);

    let mut array2: [i32; 3] = [4, 5, 6];
    println!("{}", array2[0]);
    array2[0] = 10;
}
