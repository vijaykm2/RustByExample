mod var_bindings {
    pub fn var_bindings() {
        let an_int = 1u32;
        let a_bool = true;
        let unit = ();
        let copied_int = an_int;
        println!("Copied int = {}", copied_int);
        println!("bool var = {}", a_bool);
        println!("Unit value = {:?}", unit);
        let _unused_var = 32;
    }

    pub fn mutability() {
        let _immutable_binding = 1;
        let mut _mutable_binding = 0;
        println!("Before mutation: {}", _mutable_binding);
        _mutable_binding += 1;
        println!("After mutation: {}", _mutable_binding);
        // _immutable_binding = 190;
    }

    pub fn scope_and_shadowing() {
        let long_lived_binding = 90;
        {
            let short_lived_binding = 898;
            println!("Inner short: {}", short_lived_binding);
            let long_lived_binding = 5.5f32;
            println!("Inner long: {}", long_lived_binding);
        }
        // println!("Outer short: {} ", short_lived_binding );
        println!("Outer long: {}", long_lived_binding);
        let long_lived_binding = "a";
        println!("Outer long_lived_binding: {}", long_lived_binding);
    }
}