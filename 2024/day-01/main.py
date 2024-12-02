# https://adventofcode.com/2024/day/1

import heapq

# Part 1: Given a list of pairs of numbers, find the sum of the absolute differences between the two numbers in each pair.

leftIDs = []
rightIDs = []

with open('input.txt', 'r') as file:
    lines = file.readlines()
    for line in lines:
        nums = line.split()

        heapq.heappush(leftIDs, int(nums[0]))
        heapq.heappush(rightIDs, int(nums[1]))

# https://docs.python.org/3/library/heapq.html#basic-examples
def heapsort(iterable):
    h = []
    for value in iterable:
        heapq.heappush(h, value)
    return [heapq.heappop(h) for i in range(len(h))]

leftIDs = heapsort(leftIDs)
rightIDs = heapsort(rightIDs)

# print("Left IDs: ", leftIDs)
# print("Right IDs: ", rightIDs)

if len(leftIDs) != len(rightIDs):
    print("Error: left and right IDs are not the same length")
    exit(1)

diff = 0

for i in range(len(leftIDs)):
    diff += abs(leftIDs[i] - rightIDs[i])

print(diff) # 3246517

# Part 2: Calculate a total similarity score by adding up each number in the left list after
# multiplying it by the number of times that number appears in the right list.

rightIDCount = {}
for rightID in rightIDs:
    if rightID not in rightIDCount:
        rightIDCount[rightID] = 1
    else:
      rightIDCount[rightID] += 1

score = 0
for leftID in leftIDs:
    if leftID not in rightIDCount:
        continue

    multiplier = rightIDCount[leftID]
    score += (leftID * multiplier)

print(score) # 29379307