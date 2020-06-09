mod ch2{
    use std::fmt;
    use std::mem;
    pub fn primitives(){
        let _logical: bool = true;
        let _a_float: f64 = 111.222;
        let _an_int = 5i32;
        let _default_float = 12.23;
        let _default_int = 232;
        let mut inferred_type = 123;
        inferred_type = 123123123123123i64;
        let mut mutable = 23;
        mutable = 32;
        let _mutable = false;
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
    fn analyze_slice(slice:&[i32]){
        println!("First element of slice: {}", slice[0]);
        println!("slice has {} elements", slice.len());
    }
    pub fn arrays_and_splices(){

        let xs:[i32; 5]=[1,2,3,4,78785];
        let ys:[i32; 500]=[0;500];
        // Indexing starts at 0
        println!("first element of the array: {}", xs[0]);
        println!("second element of the array: {}", xs[1]);

        // `len` returns the size of the array
        println!("array size: {}", xs.len());

        // Arrays are stack allocated
        println!("array occupies {} bytes", mem::size_of_val(&xs));
        println!("Borrow the whole array as slice \n");
        analyze_slice(&xs);

        println!("Borrow the section of array as slice\n");
        analyze_slice(&xs[1..3]);
        analyze_slice(&ys[2..10]);
        println!("{}",xs[4]);


    }
}