mod ch1 {
    pub fn print_hw() {
        println!("Hello, world!");
        let x = 5+/*90+*/5;
        println!("is `x` 10 or 100? x = {}", x);
    }
    pub fn formatted_print(){
        println!("{} days", 22);
        println!("{1} this is {0}, {0} this is {1}", "Alice", "Bob");
        println!("{subject} {verb} {object}",
            subject="the quickbrown fox", object="lazy dog", verb="jumps over"
        );
        println!("{} of {:b} people know binary, other half doesn't", 1, 22);
        println!("{num:>width$}", num=1, width = 5);
        println!("My name is {0}, {1} {0}", "Bond", "james");
        struct Str(i32);
        let asd = Str(22);
        println!("This struct won't ptint, {}", asd.0);
    }

}