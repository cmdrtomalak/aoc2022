#!/usr/bin/python3
from __future__ import annotations

class Dir:
    def __init__(self, name: str, parent: Dir | None) -> None:
        self.name = name
        self.parent = parent or self
        self.subs: dict[str, Dir] = {}
        self.files: dict[str, int] = {}

    def size(self) -> int:
        return sum(d.size() for d in self.subs.values()) + sum(self.files.values())

    def recursive_subs(self) -> set[Dir]:
        return {self}.union(*(sub.recursive_subs() for sub in self.subs.values()))

data = open('input.txt').readlines()

cwd = root = Dir('/', None)
for line in data:
    match line.split():
        case ["$", "cd", "/"]:
            cwd = root
        case ["$", "cd", ".."]:
            cwd = cwd.parent
        case ["$", "cd", sub]:
            cwd = cwd.subs.setdefault(sub, Dir(sub, cwd))
        case ["dir", sub]:
            cwd.subs[sub] = Dir(sub, cwd)
        case [size, file_name] if size.isdecimal():
            cwd.files[file_name] = int(size)

part1_result = sum(d.size() for d in root.recursive_subs() if d.size() <= 100_000)
print(f"Part 1: {part1_result}")

size_threshold = 30_000_000 - 70_000_000 + root.size()
min_ = root.size()
for d in root.recursive_subs():
    min_ = min(min_, d.size()) if d.size() >= size_threshold else min_
print(f"Part 2: {min_}")
