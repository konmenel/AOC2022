LETTER_TO_NUM = {"X": 0, "Y": 1, "Z": 2, "A": 0, "B": 1, "C": 2}
NUM_TO_LETTER = ("A", "B", "C")


def battle_outcome(your_choice: str, their_choice: str) -> int:
    """Calculated battle outcome

    Parameters
    ----------
    your_choice : str
        X, Y, or Z
    their_choice : str
        A, B, or C

    Returns
    -------
    int
        0 for lost, 3 for draw, 6 for win
    """
    score = LETTER_TO_NUM[your_choice] + 1
    difference = LETTER_TO_NUM[your_choice] - LETTER_TO_NUM[their_choice]

    return score + 6 * int(difference in (1, -2)) + 3 * int(difference == 0)


def find_play(their_choice: str, outcome: str):
    return NUM_TO_LETTER[
        ((LETTER_TO_NUM[their_choice] + (LETTER_TO_NUM[outcome] - 1)) % 3)
    ]


def main() -> int:
    score1 = 0
    score2 = 0
    with open("data/02.txt", "r") as file:
        for line in file.readlines():
            line = line.strip()
            their_choice, my_choice = line.split(" ")
            # print(battle_outcome(my_choice, their_choice))
            score1 += battle_outcome(my_choice, their_choice)

            outcome = my_choice
            my_play = find_play(their_choice, outcome)
            # print(my_play)
            score2 += battle_outcome(my_play, their_choice)

    print(score1)
    print(score2)

    return 0


if __name__ == "__main__":
    raise SystemExit(main())
