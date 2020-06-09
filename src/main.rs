use crate::ch2::primitives;
include!("ch1/helloWorld.rs");
include!("ch2/primitives.rs");
include!("ch3/custom_types.rs");
fn main() {
    ch1::print_hw();
    ch1::formatted_print();
    ch2::primitives();
    ch2::literals_and_operators();
    ch2::tuples();
    ch2::arrays_and_splices();
    custom_types::structures();
    custom_types::enums();
    custom_types::enums_use();
    custom_types::enums_c_like();
    custom_types::enums_list();
    custom_types::constants();
}
