mod expressions {
    pub fn expr() {
        let x = 5u32;
        let y = {
            let x_sq = x * x;
            let x_cube = x * x_sq;
            x_cube * x_sq * x
        };
        let z = { 2 * x; };

        println!("x is {:?}", x);
        println!("y is {:?}", y);
        println!("z is {:?}", z);
    }
}