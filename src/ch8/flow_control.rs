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

    pub fn _while() {
        let mut n = 1;
        while n < 1000 {
            fizz_buzz(n);
            n += 1;
        }
    }

    fn fizz_buzz(n: i32) {
        if n % 15 == 0 {
            println!("FizzBuzz");
        } else if n % 3 == 0 {
            println!("Fizz");
        } else if n % 5 == 0 {
            println!("Buzz");
        } else {
            println!("{}", n);
        }
    }

    pub fn for_range() {
        for n in 1..=1000 {
            fizz_buzz(n);
        }
    }

    pub fn for_iter() {
        println!("for iter");
        let mut names = vec!["Bob", "Ferris", "Frank"];

        for name in names.iter() {
            match name {
                &"Ferris" => println!("There is a rustacean among us, {}", name),

                _ => println!("Hello, {}", name)
            }
        }
        for name in names.iter_mut() {
            *name = match name {
                &mut "Ferris" => "There is a rustacean among us!",
                _ => "Hello "
            }
        }
        println!("names: {:?}", names);
    }

    pub fn _match() {
        let num = 13;
        match num {
            1 => println!("one"),
            2 | 3 | 5 | 7 | 11 | 13 | 17 | 19 => println!("Prime num"),
            13..=19 => println!("A teen"),
            _ => println!("a num = {}", num)
        }
        let boolean = true;
        let binary = match boolean {
            true => 1,
            false => 0,
        };
        println!("Binary = {}, boolean= {}", binary, boolean);
    }

    pub fn match_destructuring_tuples() {
        let pair = (12, -11);
        println!("Tell me about {:?}", pair);
        match pair {
            (x, -1) => println!("x is {} and Second is 0", x),
            (12, y) => println!("First is 0 and y is {}", y),
            _ => println!("Doesn't matter !!"),
        }
    }

    enum Color {
        RED,
        BLUE,
        GREEN,
        RGB(u32, u32, u32),
        HSV(u32, u32, u32),
        HSL(u32, u32, u32),
        CMY(u32, u32, u32),
        CMYK(u32, u32, u32, u32),
    }

    pub fn match_enums() {
        let color = Color::RGB(122, 17, 40);
        println!("What color is it ?");
        match color {
            Color::RED => println!("The color is Red!"),
            Color::BLUE => println!("The color is Blue!"),
            Color::GREEN => println!("The color is Green!"),
            Color::RGB(r, g, b) =>
                println!("Red: {}, green: {}, and blue: {}!", r, g, b),
            Color::HSV(h, s, v) =>
                println!("Hue: {}, saturation: {}, value: {}!", h, s, v),
            Color::HSL(h, s, l) =>
                println!("Hue: {}, saturation: {}, lightness: {}!", h, s, l),
            Color::CMY(c, m, y) =>
                println!("Cyan: {}, magenta: {}, yellow: {}!", c, m, y),
            Color::CMYK(c, m, y, k) =>
                println!("Cyan: {}, magenta: {}, yellow: {}, key (black): {}!",
                         c, m, y, k),
            // Don't need another arm because all variants have been examined
        }
    }

    pub fn pointers_ref() {
        let reference = &5;

        match reference {
            &val => println!("value is: {} ", &val)
        }

        match *reference {
            vaal => println!("Valueis : {}", vaal)
        }
    }
}