mod conversion {
    use std::convert::From;

    #[derive(Debug)]
    struct Number {
        value: i32
    }

    impl From<i32> for Number {
        fn from(item: i32) -> Number {
            Number { value: item }
        }
    }

    pub fn conv_from() {
        let num = Number::from(123);
        println!("Number is : {:?}", num);
    }

    pub fn conv_into() {
        let int = 5;
        let num: Number = int.into();
        println!("{} into Number is : {:?}", int, num);
    }
}