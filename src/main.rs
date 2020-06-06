include!("ch1/helloWorld.rs");
include!("ch2/primitives.rs");
fn main() {
    ch1::print_hw();
    ch1::formatted_print();
    ch2::primitives();
    ch2::literals_and_operators();
    ch2::tuples();
    ch2::arrays_and_splices();
}
