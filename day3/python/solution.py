#!/usr/bin/python3

from math import floor

def read_file(filename:str) -> [str]:
    out = []
    with open(filename, 'r') as file:
        for line in file:
            out.append(line.strip())

    return out


def get_common_item(data: [str]) -> [str]:
    total_common = []
    for s in data:
        s1 = set(s[:int(len(s)/2)])
        s2 = set(s[int(len(s)/2):])
        # intersection of set
        common = s1 & s2

        total_common += [ch for ch in common]

    return total_common


def game_part1(common: [str]) -> int:
    sum_of_priorities = 0
    for ch in common:
        if ch.isalpha():
            if ch.islower():
                # a-z is 97 to 122 in ascii
                # Lowercase letters have priorities 1-26
                sum_of_priorities += ord(ch) - 96
            else:
                # A-Z is 65 to 90 in ascii
                # Capital letters have priorities 27 to 52
                sum_of_priorities += ord(ch) - 64 + 26

    return sum_of_priorities


def game_part2(data: [str]) -> int:
    sum_of_priorities = 0

    for i in range(0, len(data), 3):
        elf_group = data[i: i+3]

        badge = [x for x in elf_group[0] if x in elf_group[1] and x in elf_group[2]][0]

        if badge.isalpha():
            if badge.islower():
                # a-z is 97 to 122 in ascii
                # Lowercase letters have priorities 1-26
                sum_of_priorities += (ord(badge) - 96)
            else:
                # A-Z is 65 to 90 in ascii
                # Capital letters have priorities 27 to 52
                sum_of_priorities += (ord(badge) - 64 + 26)

    return sum_of_priorities


if __name__ == '__main__':
    data = read_file('input.txt')
    common = get_common_item(data)
    print(game_part1(common))
    print(game_part2(data))

