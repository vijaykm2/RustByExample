mod ch2{
    use std::fmt;
    use std::mem;
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
    fn reverse(pair:(i32, bool)) -> (bool, i32){
        let (integer, boolean)= pair;
        (boolean, integer)
    }
    // The following struct is for the activity.
    #[derive(Debug)]
    struct Matrix(f32, f32, f32, f32);
    impl fmt::Display for Matrix{
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f,"\n ({} {})\n ({} {})", self.0, self.1, self.2, self.3)
        }
    }
    fn transpose(matrix:Matrix) -> Matrix{
        Matrix(matrix.0,matrix.2,matrix.1,matrix.3)
    }
    pub fn tuples(){
        let long_tuple = (1u8, 2u16, 3u32, 4u64,
                          -1i8, -2i16, -3i32, -4i64,
                          0.1f32, 0.2f64,
                          'a', true);

        // Values can be extracted from the tuple using tuple indexing
        println!("long tuple first value: {}", long_tuple.0);
        println!("long tuple second value: {}", long_tuple.1);

        // Tuples can be tuple members
        let tuple_of_tuples = ((1u8, 2u16, 2u32), (4u64, -1i8), -2i16);

        // Tuples are printable
        println!("tuple of tuples: {:?}", tuple_of_tuples);

        // But long Tuples cannot be printed
        // let too_long_tuple = (1, 2, 3, 4,4,3,6, 3,1,2,4,8,2,3,4);
        // println!("too long tuple: {:?}", too_long_tuple);
        // TODO ^ Uncomment the above 2 lines to see the compiler error

        let pair = (1, true);
        println!("pair is {:?}", pair);

        println!("the reversed pair is {:?}", reverse(pair));

        // To create one element tuples, the comma is required to tell them apart
        // from a literal surrounded by parentheses
        println!("one element tuple: {:?}", (5u32,));
        println!("just an integer: {:?}", (5u32));

        //tuples can be destructured to create bindings
        let tuple = (1, "hello", 4.5, true);

        let (a, b, c, d) = tuple;
        println!("{:?}, {:?}, {:?}, {:?}", a, b, c, d);

        let matrix = Matrix(1.1, 1.2, 2.1, 2.2);
        println!("{:?}", matrix);
        println!("Display: {}", matrix);
        println!("transpose Display: {}", transpose(matrix));

    }
}