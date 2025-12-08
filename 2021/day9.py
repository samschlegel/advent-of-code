import time
from typing import Counter, Optional


heightmap = []
with open('inputs/day9.txt') as f:
  for l in f:
    row = []
    for c in l.strip():
      row.append(int(c))
    heightmap.append(row)

map_height = len(heightmap)
map_width = len(heightmap[0])

def get_height(x, y) -> Optional[int]:
  if x < 0 or y < 0:
    return 9999999999999

  try:
    return heightmap[y][x]
  except IndexError:
    return 9999999999999

def propagate_basin(x, y, basin):
  global new_basins
  if x < 0 or y < 0:
    return
  try:
    if basins[y][x] is None and heightmap[y][x] != 9:
      new_basins[y][x] = basin
  except IndexError:
    pass

low_points = []

for y in range(0, map_height):
  for x in range(0, map_width):
    height = get_height(x, y)
    if get_height(x + 1, y) <= height:
      continue
    elif get_height(x - 1, y) <= height:
      continue
    elif get_height(x, y + 1) <= height:
      continue
    elif get_height(x, y - 1) <= height:
      continue
    low_points.append((x, y, height))

sum = 0
for (x, y, height) in low_points:
  sum += height + 1

print(sum)

basins = []
for y in range(0, map_height):
  row = []
  for x in range(0, map_width):
    row.append(None)
  basins.append(row)

def print_basins():
  print('-' * map_width)
  for y in range(0, map_height):
    print('|', end='')
    for x in range(0, map_width):
      basin = basins[y][x]
      if basin is None:
        print(' ', end='')
      else:
        print(basin, end='')
    print('|')
  print('-' * map_width)

print(low_points)

for (i, (x, y, height)) in enumerate(low_points):
  print(x, y, i)
  basins[y][x] = i
# print_basins()



while True:
  done = True
  # print_basins()

  new_basins = [[basin for basin in row] for row in basins]
  for y in range(0, map_height):
    for x in range(0, map_width):
      height = heightmap[y][x]
      basin = basins[y][x]
      if height == 9:
        continue
      elif basin is None:
        done = False

      if basin is None:
        continue

      if get_height(x + 1, y) > height:
        propagate_basin(x + 1, y, basin)
      if get_height(x - 1, y) > height:
        propagate_basin(x - 1, y, basin)
      if get_height(x, y + 1) > height:
        propagate_basin(x, y + 1, basin)
      if get_height(x, y - 1) > height:
        propagate_basin(x, y -1, basin)

  basins = new_basins
  if done:
    break

basin_sizes = Counter()
for y in range(0, map_height):
  for x in range(0, map_width):
    basin = basins[y][x]
    if basin is None:
      continue
    basin_sizes[basin] += 1

print(basin_sizes.most_common())

m = 1
for (basin, count) in basin_sizes.most_common()[:3]:
  m *= count

print(m)
