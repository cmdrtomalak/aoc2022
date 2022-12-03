#!/usr/bin/python3

max_cals = 0
cals_of_current_inventory = 0
with open('elf_calories.txt', 'r') as f:
    for line in f:
        if line == '\n':
            max_cals = max(max_cals, cals_of_current_inventory)
            cals_of_current_inventory = 0
            continue
        try:
            cals = int(line)
        except Exception:
            raise Exception(f'line {line} not a valid positive number')
        
        cals_of_current_inventory += cals 
print(maxCals)
