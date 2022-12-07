#!/usr/bin/python3

def read_file(filename: str) -> list(tuple(set())):
    out = []
    with open(filename, 'r') as file:
        for line in file:
            (s1, s2) = line.strip().split(',')
            (n1, n2) = s1.split('-')
            (n3, n4) = s2.split('-')
            section1 = set(range(int(n1), int(n2)+1))
            section2 = set(range(int(n3), int(n4)+1))
            out.append((section1, section2))
    return out
    

def part1(data: list(tuple(set()))) -> int:
    sum = 0
    for pair in data:
        if pair[0].issubset(pair[1]) or pair[1].issubset(pair[0]):
            sum += 1
    
    return sum


def part2(data: list(tuple(set()))) -> int:
    sum = 0
    for pair in data:
        if pair[0] & pair[1]:
            sum += 1

    return sum


if __name__ == "__main__":
    data = read_file("input.txt")
    print(part1(data))
    print(part2(data))
