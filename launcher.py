import os


if __name__ == "__main__":

    program_path = os.path.join(
        os.path.dirname(__file__), "target/debug/computorv1"
    )

    # Arguments to pass to the program
    arg_list = (

        # Valid 0 degree equation
        # with solutions
        "42 = 42",
        "-42 = -42",
        "-42.5 = -42.5",
        "21 + 21 = 42",
        # without solutions
        "42 = 0",
        "-42 = 21",
        "-42.5 = 0",
        "21 - 21 = 42",

        # Valid 1 degree equation

        # Valid 2 degree equation

        "5 * X^0 + 4 * X^1 - 9.3 * X^2 = 1 * X^0",
        "5 * X^0 + 4 * X^1 = 4 * X^0",
        # "8 * X^0 - 6 * X^1 + 0 * X^2 - 5.6 * X^3 = 3 * X^0",
        "5 + 4 * X + X^2= X^2"
    )

    for arg in arg_list:

        print(f"Running: {program_path} {arg}")
        os.system("cargo run -- '" + arg + "'")

        while True:
            input_value = input("\nContinue? (y/n): ").capitalize()
            if input_value == "Y":
                break
            else:
                exit()
