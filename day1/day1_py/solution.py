maxCals = 0
calsOfCurrentReindeer = 0
with open('elf_calories.txt', 'r') as f:
    for line in f:
        if line == '\n':
            maxCals = max(maxCals, calsOfCurrentReindeer)
            calsOfCurrentReindeer = 0
            continue
        try:
            cals = int(line)
        except Exception:
            raise Exception(f'line {line} not a valid positive number')
        
        calsOfCurrentReindeer += cals 
print(maxCals)
