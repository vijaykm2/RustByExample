mod custom_types {
    // use crate::List::*;
    use crate::custom_types::Status::Poor;
    use crate::custom_types::Work::Civilian;
    use crate::custom_types::List::{Nil, Cons};

    #[derive(Debug)]
    struct Person<'a> {
        name: &'a str,
        age: u8,
    }

    struct Unit;

    struct Pair(i32, f32);

    struct Point {
        x: f32,
        y: f32
    }
    #[allow(dead_code)]
    struct Rectangle {
        top_left: Point,
        bottom_right: Point
    }
    pub fn structures (){
        let name="peter";
        let age=27;
        let peter = Person{name, age};
        println!("{:?}", peter);
        let _point =Point{ x: 12 as f32, y: 21 as f32 };
        println!("point coords: x = {}, y= {}", _point.x, _point.y);
        let bottom_right: Point=Point{x:5.5, .._point};
        println!("bottom_right = x = {}, y = {}", bottom_right.x, bottom_right.y);
        let Point { x: top_edge, y: left_edge } = _point;
        let _rectangle = Rectangle {
            top_left: Point{x: left_edge, y: top_edge},
            bottom_right
        };
        let unit:Unit;
        let pair=Pair(1, 2.2);
        println!("Pair contains {:?}, {:?}", pair.0, pair.1);
        let Pair(integer, decimal) = pair;
        println!("pair contains {}, {}", integer, decimal);


    }
    enum WebEvent {
        PageLoad,
        PageUnLoad,
        KeyPress(char),
        Paste(String),
        Click{x: i64, y:i64}
    }
    fn inspect(event: WebEvent){
        match event {
            WebEvent::PageLoad => println!("Page Loaded!!"),
            WebEvent::PageUnLoad => println!("Page unloaded !!"),
            WebEvent::KeyPress(c) => println!("{} key pressed.", c),
            WebEvent::Click {x, y} => println!("clicked at {}, {} coordinates", x, y),
            WebEvent::Paste(str) => println!("Pasted {}", str)
        }
    }
    pub fn enums(){
        inspect(WebEvent::PageLoad);
        inspect(WebEvent::KeyPress('x'));
        inspect(WebEvent::Paste("Vijay krishna".to_owned()));
        inspect(WebEvent::Click { x: 123, y:3432 });
        inspect(WebEvent::PageUnLoad);
    }
    enum
    Status{
        Rich, Poor
    }
    enum Work {
        Civilian, Soldier
    }
    pub fn enums_use(){
        use Status::{Poor, Rich};
        use Work::*;

        let status= Poor;
        let work=Civilian;
        match status {
            Rich => println!("Rich"),
            Poor => println!("Poor"),
        }
        match work {
            Soldier => println!("Soldier"),
            Civilian => println!("civilian")
        }
    }

    enum Number{
        Zero,
        One,
        Two
    }
    enum Color{
        Red=0xff0000,
        Green=0x00ff00,
        Blue=0x0000ff
    }
    pub fn enums_c_like(){
        println!("zero is {}", Number::Zero as i32);
        println!("One is {}", Number::One as i32);
        println!("two is {}", Number::Two as i32);

        println!("Roses are {}", Color::Red as i32);
        println!("Violets are {}", Color::Blue as i32);
        println!("Grass is {}", Color::Green as i32);
    }

    enum List{
        Cons(u32, Box<List>),
        Nil
    }
    impl List {
        fn new() -> List{
            Nil
        }
        fn prepend(self, elem: u32) ->List{
            Cons(elem, Box::new(self))
        }
        fn len(&self) -> u32{
            match self {
                Cons(_, ref tail) => 1+tail.as_ref().len(),
                Nil => 0
            }
        }

        fn stringify(&self) -> String{
            match *self {
                Cons(head, ref tail) => format!("{}, {}", head, tail.stringify()),
                Nil => format!("Nil")
            }
        }
    }
    pub fn enums_list(){
        let mut list = List::new();
        list = list.prepend(1);
        list = list.prepend(2);
        list = list.prepend(3);
        println!("Linked list has length: {}", list.len());
        println!("{}", list.stringify());
    }

    static LANGUAGE: &str="Rust";
    const THRESHOLD: i32=0;
    fn is_big(num:i32) -> bool{
        num > THRESHOLD
    }
    pub fn constants(){
        let num = 121;

        // Access constant in the main thread
        println!("This is {}", LANGUAGE);
        println!("The threshold is {}", THRESHOLD);
        println!("{} is {}", num, if is_big(num) { "big" } else { "small" });
       // THRESHOLD = 12;
    }
}