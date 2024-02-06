import sys,re
import heapq

bots = [map(int, re.findall("-?\d+", line)) for line in sys.stdin]
q = []
for x,y,z,r in bots:
    d = abs(x) + abs(y) + abs(z)
    heapq.heappush(q, (max(0, d - r),1))
    heapq.heappush(q, (d + r + 1,-1))

count = 0
maxCount = 0
result = 0
while q:
    dist,e = heapq.heappop(q)
    count += e
    if count > maxCount:
        result = dist
        maxCount = count
print(result)