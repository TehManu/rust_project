use crate::data_types::{data_types_array, data_types_array_accessing,
                        data_types_array_invalid_access, data_types_bool,
                        data_types_char, data_types_floating,
                        data_types_numeric_operations, data_types_tuple};
use crate::variables::{variables_const, variables_let, variables_mut,
                       variables_shadowing, variables_shadowing_2};

mod variables;
mod data_types;

fn main() {
    variables();
    data_types_floating();
    data_types_numeric_operations();
    data_types_bool();
    data_types_char();
    data_types_tuple();
    data_types_array();
    data_types_array_accessing();
    data_types_array_invalid_access();
}

fn variables() {
    variables_let();
    variables_mut();
    variables_const();
    variables_shadowing();
    variables_shadowing_2();
}
