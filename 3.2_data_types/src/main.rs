fn main() {
    let age : i8 = -128;
    println!("This is the age {}", age);

    let age : u8 = 255;
    //let age : u8 = -10; //not possible
    println!("This is the age {}", age);

    let _id : i16 = -32768;
    let _id : u16 = 65535;
    println!("This is the _id {}", _id);

    let _new_id : i32 = -2147483648;
    let _new_id : u32 = 4294967295;
    println!("This is the _new_id {}", _new_id);

    let _new_id : i64 = -9223372036854775808;
    let _new_id : u64 = 18446744073709551615;
    println!("This is the _new_id {}", _new_id);

    let _new_id : i128 = -170141183460469231731687303715884105728;
    let _new_id : u128 = 340282366920938463463374607431768211455;
    println!("This is the _new_id {}", _new_id);

    // Depends on your processor architecture.
    let _new_id : isize = isize::MIN;
    let _new_id : usize = usize::MAX;
    println!("This is the _new_id {}", _new_id);

    let decimal = 98_222;
    let hex = 0xff;
    let octal = 0o77;
    let binary = 0b0000_0001;
    let byte = b'A';
    println!("Decimal sample {}", decimal);
    println!("Hex sample {}", hex);
    println!("Octal sample {}", octal);
    println!("Binary sample {}", binary);
    println!("Byte sample {}", byte);

    let x = 2.1; //f64
    let y :f32 = 15.1;
    println!("({} , {})", x, y);

    let _alive = true;
    let _alive : bool = false;
    println!("Am I alive? {}", _alive);


    let tup : (u32, &str, bool) = (1, "Hey", false);
    println!("This could be a touple {:?}", tup);

    let (_id, _phrase, _alive) = tup;
    println!("Id: {}", _id);
    println!("Phrase: {}", _phrase);
    println!("Alive: {}", _alive);

    let john : (u32, &str, bool) = tup;
    println!("Id: {}", john.0);
    println!("Phrase: {}", john.1);
    println!("Alive: {}", john.2);

    const _MONTHS: [&str; 12] = ["January", "February", "March", "April", "May", "June", "July",
                "August", "September", "October", "November", "December"];
    println!("Months: {:?}", _MONTHS);

    let _arr = [3; 5];
    println!("Array: {:?}", _arr);

    let _arr: [i32; 5] = [1, 2, 3, 4, 5];
    println!("Array: {:?}", _arr);

    println!("2nd Months: {}", _MONTHS[1]);
}
