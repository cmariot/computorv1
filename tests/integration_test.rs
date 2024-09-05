use computorv1::run;

#[test]
fn computorv1_tests() {
    let second_arg = [
        "1 * X^0 + 1 * X^1 + 1 * X^2 = 0",
        "- 1 * X^0 - 1 * X^1 - 1 * X^2 = 0",
    ];

    // Launch the run function for each element of the second_arg tuple
    // Set the env::args() as if it was launched from the command line with the second_arg as argument and the first_arg as program name

    for arg in second_arg.iter() {
        run(String::from(*arg));
    }
}
