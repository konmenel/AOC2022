def main() -> int:
    def _update_logic():
        if current_cals > most_cals[0]:
            most_cals[2] = most_cals[1]
            most_cals[1] = most_cals[0]
            most_cals[0] = current_cals

            most_cals_elfs[2] = most_cals_elfs[1]
            most_cals_elfs[1] = most_cals_elfs[0]
            most_cals_elfs[0] = current_elf

        elif current_cals > most_cals[1]:
            most_cals[2] = most_cals[1]
            most_cals[1] = current_cals

            most_cals_elfs[2] = most_cals_elfs[1]
            most_cals_elfs[1] = current_cals

        elif current_cals > most_cals[2]:
            most_cals[2] = current_cals

            most_cals_elfs[2] = current_elf

    current_elf = 1
    current_cals = 0
    most_cals = [0, 0, 0]
    most_cals_elfs = [0, 0, 0]

    with open("data/01.txt", "r") as fin:
        for line in fin.readlines():
            line = line.strip()

            if not line:
                print(current_cals)
                _update_logic()

                current_cals = 0
                current_elf += 1
                continue

            current_cals += int(line)

    _update_logic()
    print(most_cals, most_cals_elfs)
    print(most_cals[0] + most_cals_elfs[1] + most_cals_elfs[2])
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
