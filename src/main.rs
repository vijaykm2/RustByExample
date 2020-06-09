use crate::ch2::primitives;
include!("ch1/helloWorld.rs");
include!("ch2/primitives.rs");
include!("ch3/custom_types.rs");
include!("ch4/variable_bindings.rs");
fn main() {
    ch1::print_hw();
    ch1::formatted_print();
    ch2::primitives();
    ch2::literals_and_operators();
    ch2::tuples();
    ch2::arrays_and_splices();
    // Chapter 3
    custom_types::structures();
    custom_types::enums();
    custom_types::enums_use();
    custom_types::enums_c_like();
    custom_types::enums_list();
    custom_types::constants();
    // Chapter 4
    var_bindings::var_bindings();
    var_bindings::mutability();
    var_bindings::scope_and_shadowing();
}
