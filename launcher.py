import os


if __name__ == "__main__":

    program_path = os.path.join(
        os.path.dirname(__file__), "target/debug/computorv1"
    )

    # Arguments to pass to the program
    arg_list = (
        "5 * X^0 + 4 * X^1 - 9.3 * X^2 = 1 * X^0",
        "5 * X^0 + 4 * X^1 = 4 * X^0",
        # "8 * X^0 - 6 * X^1 + 0 * X^2 - 5.6 * X^3 = 3 * X^0",
        "5 + 4 * X + X^2= X^2"
    )

    for arg in arg_list:
        print(f"Running: {program_path} {arg}")
        return_value = os.system("cargo run '" + arg + "'")
        if return_value != 0:
            print("Error running program")
            break

        while True:
            if input("Run again? (y/n): ") != "y":
                break




