use crate::ch2::primitives;
use crate::conversion::{conv_from, conv_into, try_from_into, conv_to_string};
include!("ch1/helloWorld.rs");
include!("ch2/primitives.rs");
include!("ch3/custom_types.rs");
include!("ch4/variable_bindings.rs");
include!("ch5/types.rs");
include!("ch6/conversion.rs");
include!("ch7/expressions.rs");
include!("ch8/flow_control.rs");
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
    var_bindings::declare_first();
    var_bindings::freezing();
    //chapter 5
    types::casting();
    types::literals();
    types::inference();
    types::aliasing();

    //chapter 6
    conv_from();
    conv_into();
    try_from_into();
    conv_to_string();

    //ch7
    expressions::expr();
    flow_control::if_else();
    flow_control::_loop();
}
