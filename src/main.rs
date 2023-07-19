use crate::variables::{variables_const, variables_let, variables_mut, variables_shadowing, variables_shadowing_2};

mod variables;

fn main() {
    variables_let();
    variables_mut();
    variables_const();
    variables_shadowing();
    variables_shadowing_2();
}
