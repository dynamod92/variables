mod arrays;
mod conditionals;
mod loops;
mod numbers;
mod slices;
mod strings;
mod temp;
mod vars;

mod chapter_four; // this pulls in the "chapter_four" mod via its mod.rs file

// use crate::chapter_four::borrowing; // this lets us actually use the packages exported from the module
use crate::chapter_four::ownership; // i guess you need one per-file?
use crate::chapter_four::rust_in_ten; // yep you definitely need one per file lol

fn main() {
    conditionals::ternary_test();

    vars::const_vs_let();
    vars::passing_values_to_fn(5, '🎲');

    numbers::number_examples();
    numbers::math_examples();

    strings::string_examples();
    strings::char_examples();

    arrays::tuples_vs_arrays();

    loops::looping_with_loop();
    loops::looping_with_while();
    loops::looping_over_collection();
    loops::looping_over_range();

    temp::c_to_f(2.0);
    temp::f_to_c(57.0);

    slices::slice_array();

    // using imported crates/mods
    // borrowing::hello_from_file();
    chapter_four::borrowing::hello_from_file();

    ownership::i_own_this();
    ownership::try_to_ref_old_var();
    ownership::clone_old_var();
    ownership::copy_number();

    rust_in_ten::filter_lowercase();
    rust_in_ten::iterate_over_vec();
    rust_in_ten::iterate_over_char_vec();
    rust_in_ten::use_index_map(3);
    rust_in_ten::enumerate_over_char_vec();
}
