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
}