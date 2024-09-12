import os


if __name__ == "__main__":

    program_path = os.path.join(
        os.path.dirname(__file__), "target/debug/computorv1"
    )

    # Arguments to pass to the program
    arg_list = (

        # # Valid 0 degree equation
        # # with solutions
        # "42 = 42",
        # "-42 = -42",
        # "-42.5 = -42.5",
        # "21 + 21 = 42",
        # # without solutions
        # "42 = 0",
        # "-42 = 21",
        # "-42.5 = 0",
        # "21 - 21 = 42",

        # # Valid 1 degree equation

        # # Valid 2 degree equation

        # "5 * X^0 + 4 * X^1 - 9.3 * X^2 = 1 * X^0",
        # "5 * X^0 + 4 * X^1 = 4 * X^0",
        # # "8 * X^0 - 6 * X^1 + 0 * X^2 - 5.6 * X^3 = 3 * X^0",
        # "5 + 4 * X + X^2= X^2",

        "5 * X^0 + 4 * X^1 - 9.3 * X^2 = 1 * X^0",   # Cas valide
        "5 X^1 + 3 * X^0 = 2 * X",                   # Cas valide: Coefficients implicites
        "4 * X + 7 = X^2 + 3X",                      # Cas valide: coefficients et puissances implicites
        "3 * X^ + 4 * X^1 = 2 * X^0",                # Erreur: Puissance manquante
        "X^2 + X = ",                                # Erreur: Expression incomplète après le signe "="
        "= 3 * X^0 + 5 * X",                         # Erreur: Manque l'expression avant le "="
        "3 ** X^1 + 2 * X^2 = X^0",                  # Erreur: Double "*"
        "X^0 + X^1 = X^2 + X^-1",                    # Erreur: Puissance négative
        "4 * Y^2 + 2 = 5 * Y",                       # Erreur: Variable inconnue "Y"
        "5 * X^1.5 + 2 = X",                         # Erreur: Puissance décimale non supportée
        " = ",                                       # Erreur: Équation vide
        "5 * X^0 + 4 * X^1",                         # Erreur: Manque la partie droite de l'équation
        "5 * X^2 = 3 * X^1 = 1 * X^0",               # Erreur: Plusieurs signes "="
        "5 * X^0 + 4 * X^1 - -3 * X^2 = 1 * X^0",    # Double signe "-" avant un coefficient
        "5 / X^0 + 4 * X^1 = 1 * X^0",               # Erreur: "/" au lieu de "*"
        "X^2 + X = X^",                              # Erreur: Manque la puissance
        "X^2 + X^1.5 = X",                           # Erreur: Puissance décimale non supportée
        "5 * (X^2 + X = 1 * X)",                     # Erreur: Parenthèses mal placées
        "5 * X^2 + 4@ * X = 2 * X^0",                # Caractère spécial non valide "@"
        "5 * X^1 + 3 * Y^0 = 2 * X^2",               # Utilisation incohérente des variables

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
