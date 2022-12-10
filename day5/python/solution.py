#!/usr/bin/python3

def read_file(filename: str) -> ([str], [str]):
    with open(filename, 'r') as file:
        crate, algo = file.read().split('\n\n')
        crate = crate.split('\n')
        algo = algo.split('\n')

        return (crate, algo)


if __name__ == '__main__':
    (crate, algo) = read_file('input.txt')
    print(algo)
