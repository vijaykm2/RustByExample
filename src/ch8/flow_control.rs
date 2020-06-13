mod flow_control {
    pub fn if_else() {
        let n = 25;
        if n < 0 {
            println!("n is negative");
        } else if n > 0 {
            println!("n is positive");
        } else {
            println!("n is zero");
        }
        let bign = {
            if n > -10 && n < 10 {
                n * 10
            } else {
                n / 10
            }
        };
        println!("bign is {}", bign);
    }

    pub fn _loop() {
        let mut count = 0u32;
        println!("Lets count till infinity!!");
        loop {
            count += 1;
            if count == 3 {
                println!("Three");
                continue;
            }
            println!("{}", count);
            if count == 5 {
                println!("Thats enough!!");
                break;
            }
        }

        'outer: loop {
            println!("Entered outer loop");
            'inner: loop {
                println!("Entered inner loop");
                break 'outer;
            }
            println!("This is not reached");
        }
        println!("Exited outer loop");

        let mut counter = 0;
        let result = loop {
            counter += 1;
            if counter == 10 {
                break counter * 2;
            }
        };
        assert_eq!(result, 20)
    }
}