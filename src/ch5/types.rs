mod types {
    // Suppress all warnings from casts which overflow.
    #![allow(overflowing_literals)]

    pub fn casting() {
        let decimal: f32 = 62515.2837;
        let integer: u8 = decimal as u8;
        let character: char = integer as char;
        let char2: char = 123 as char;
        println!("Casting {} -> {}, {}, {}", decimal, integer, character, char2);
        println!("1000 as u16 is {}", 1000 as u16);
        println!("1000 as u8 is {}", 1000 as u8);
        // -1 + 256 = 255
        println!("  -1 as a u8 is : {}", (-1i8) as u8);

        // For positive numbers, this is the same as the modulus
        println!("1000 mod 256 is : {}", 1000 % 256);

        // When casting to a signed type, the (bitwise) result is the same as
        // first casting to the corresponding unsigned type. If the most significant
        // bit of that value is 1, then the value is negative.

        // Unless it already fits, of course.
        println!(" 128 as a i16 is: {}", 128 as i16);
        // 128 as u8 -> 128, whose two's complement in eight bits is:
        println!(" 128 as a i8 is : {}", 128 as i8);

        // repeating the example above
        // 1000 as u8 -> 232
        println!("1000 as a u8 is : {}", 1000 as u8);
        // and the two's complement of 232 is -24
        println!(" 232 as a i8 is : {}", 232 as i8);
    }

    pub fn literals() {
        let x = 1u8;
        let y = 2u32;
        let z = 3f32;
        let i = 1;
        let f = 1.0;
        // `size_of_val` returns the size of a variable in bytes
        println!("size of `x` in bytes: {}", std::mem::size_of_val(&x));
        println!("size of `y` in bytes: {}", std::mem::size_of_val(&y));
        println!("size of `z` in bytes: {}", std::mem::size_of_val(&z));
        println!("size of `i` in bytes: {}", std::mem::size_of_val(&i));
        println!("size of `f` in bytes: {}", std::mem::size_of_val(&f));
    }

    pub fn inference() {
        let elem = 8u8;
        let mut vec = Vec::new();
        vec.push(elem);
        println!("vec = {:?}", vec);
        // vec.push("asd");
    }

    pub fn aliasing() {
        type NanoSecond = u64;
        type Inch = u64;
        type u64_t = u64;
        let nanos: NanoSecond = 5 as u64_t;
        let inches: Inch = 3 as u64_t;
        println!("Nanosecs = {}, inches= {}", nanos, inches);
    }
}