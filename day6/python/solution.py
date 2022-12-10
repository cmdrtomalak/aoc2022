#!/usr/bin/python3

file = open('input.txt', 'r')
data = file.read()

for i in range(3, len(data)):
  if len(set(data[i-3:i+1])) == 4:
    print('Part 1: ', i+1)
    break

for i in range(13, len(data)):
  if len(set(data[i-13:i+1])) == 14:
    print('Part 2: ', i+1)
    break
