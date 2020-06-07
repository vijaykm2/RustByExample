mod custom_types {
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
}