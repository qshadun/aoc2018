import re, sys
from collections import defaultdict

nanobots = [[int(x) for x in re.findall("-?\d+", line)] for line in sys.stdin]
nanobots = [((n[0], n[1], n[2]), n[3]) for n in nanobots]

def dist(c0, c1):
    (x0, y0, z0) = c0
    (x1, y1, z1) = c1
    return abs(x0-x1) + abs(y0-y1) + abs(z0-z1)

srad = 0
rad_idx = 0
in_range = defaultdict(int)
for i in range(len(nanobots)):
    pos, rng = nanobots[i]
    strength = 0
    if rng > srad:
        srad = rng
        rad_idx = i
        for j in range(len(nanobots)):
            npos, _ = nanobots[j]
            if dist(pos, npos) <= rng:
                in_range[i] += 1

print ("a", in_range[rad_idx])

from z3 import *
def zabs(x):
    return If(x >= 0,x,-x)
(x, y, z) = (Int('x'), Int('y'), Int('z'))
in_ranges = [
    Int('in_range_' + str(i)) for i in range(len(nanobots))
]
range_count = Int('sum')
o = Optimize()
for i in range(len(nanobots)):
    (nx, ny, nz), nrng = nanobots[i]
    o.add(in_ranges[i] == If(zabs(x - nx) + zabs(y - ny) + zabs(z - nz) <= nrng, 1, 0))
o.add(range_count == sum(in_ranges))
dist_from_zero = Int('dist')
o.add(dist_from_zero == zabs(x) + zabs(y) + zabs(z))
h1 = o.maximize(range_count)
h2 = o.minimize(dist_from_zero)
print(o.check())
#print o.lower(h1)
#print o.upper(h1)
print("b", o.lower(h2), o.upper(h2))
#print o.model()[x]
#print o.mode()[y]
#print o.model()[z]