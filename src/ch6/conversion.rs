mod conversion {
    use std::convert::From;
    use std::convert::TryFrom;
    use std::convert::TryInto;
    use std::fmt;
    use std::fmt::Formatter;

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

    #[derive(Debug, PartialEq)]
    struct EvenNumber(i32);

    impl TryFrom<i32> for EvenNumber {
        type Error = ();
        fn try_from(value: i32) -> Result<Self, Self::Error> {
            if value % 2 == 0 {
                Ok(EvenNumber(value))
            } else {
                Err(())
            }
        }
    }

    pub fn try_from_into() {
        assert_eq!(EvenNumber::try_from(8), Ok(EvenNumber(8)));
        assert_eq!(EvenNumber::try_from(3), Err(()));

        let result: Result<EvenNumber, ()> = 8.try_into();
        assert_eq!(result, Ok(EvenNumber(8)));
        let result2: Result<EvenNumber, ()> = 9.try_into();
        assert_eq!(result2, Err(()));
    }

    struct Circle {
        radius: i32
    }

    impl fmt::Display for Circle {
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
            write!(f, "Circle of radius: {}", &self.radius)
        }
    }

    pub fn conv_to_string() {
        let circle = Circle { radius: 23 };
        println!("{}", circle.to_string());
    }
}