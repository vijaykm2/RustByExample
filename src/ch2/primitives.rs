mod ch2{
    pub fn primitives(){
        let logical:bool = true;
        let a_float: f64=111.222;
        let an_int = 5i32;
        let default_float=12.23;
        let default_int=232;
        let mut inferred_type=123;
        inferred_type=123123123123123i64;
        let mut mutable=23;
        mutable=32;
        let mutable=false;
    }

    pub fn literals_and_operators(){
        println!("1+2 = {}", 1u32+2u32);
        println!("1-2 = {}", 1i32-2i32);
        println!("true && false is {}", true&&false);
        println!("true || false is {}", true||false);
        println!("!true is {}", !true);
        println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);
        println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);
        println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);
        println!("1 << 5 is {}", 1u32 << 5);
        println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);

        // Use underscores to improve readability!
        println!("One million is written as {}", 1_000_000u32);
    }
    pub fn tuples(){

    }
}