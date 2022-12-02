elf_totals = []
current_total = 0

with open('input/day1.txt') as f:
    for line in f:
        try:
            current_total += int(line)
        except Exception:
            elf_totals.append(current_total)
            current_total = 0

print(f'Part 1: {max(elf_totals)}')

top_three = sorted(elf_totals)[-3:]
print(f'Part 2: {sum(top_three)}')