#!/usr/bin/python3

def read_file(filename: str) -> ([str], [str]):
    with open(filename, 'r') as file:
        crate, algo = file.read().split('\n\n')
        crate = crate.strip().split('\n')
        algo = algo.strip().split('\n')

        return (crate, algo)


def parse_crate(crate: [str]) -> [str]:
    stacks = [""]*10
    for line in crate[:-1]:
        for i, box in enumerate(line[1::4]):
            if box != " ": stacks[i+1] += box

    return stacks

    
if __name__ == '__main__':
    (crate, algo) = read_file('input.txt')
    stacks = parse_crate(crate)

    part1 = stacks[:]
    part2 = stacks[:]
    for line in algo:
        _, n, _, src, _, dest = line.split()
        n = int(n); src = int(src); dest = int(dest)
    
        part1[src], part1[dest] = part1[src][n:], part1[src][:n][::-1] + part1[dest]
        part2[src], part2[dest] = part2[src][n:], part2[src][:n] + part2[dest]
    
    print("Part 1:", "".join(s[0] for s in part1 if s))
    print("Part 2:", "".join(s[0] for s in part2 if s))
