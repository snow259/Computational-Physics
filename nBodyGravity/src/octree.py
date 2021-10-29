class octree:

	def __init__(self, center, width, capacity, depth=0, maxDepth=10):
		# Define position and size
		self.center = center
		self.width = width

		# Define capacity of each octant
		self.capacity = capacity

		# Mass contained in octant, and position of center of mass
		self.mass = 0
		self.com = center.copy()

		# Node depth
		self.maxDepth = 10
		self.depth = depth

		# Contents of octants
		# Right handed axis, x right, y upwards, z towards
		# 0: +++ : right, top, front
		# 1: ++- : right, top, rear
		# 2: +-+ : right, bot, front
		# 3: +-- : right, bot, rear
		# 4: -++ : left, top, front
		# 5: -+- : left, top, rear
		# 6: --+ : left, bot, front
		# 7: --- : left, bot, rear
		# 8 nested lists, because initialising with [] * 8 will create 8 references
		# to the same object in memory
		self.octants = [[], [], [], [], [], [], [], []]
		self.contained = [0] * 8

	def insert(self, point):
		# Find octant to insert point, check octant for capacity
		octantIndex = self.findOctant(point)

		if self.contained[octantIndex] < self.capacity:
			# Insert point to octant
			self.octants[octantIndex].append(point)
		elif self.contained[octantIndex] == self.capacity:
			# If octant is at capacity, insert point, partition
			self.octants[octantIndex].append(point)
			self.partition(octantIndex)
		else:
			# Octant already partitioned, insert to octree in octant
			self.octants[octantIndex].insert(point)

		newMass = self.mass + point[3]
		for i in range(len(self.com)):
			self.com[i] = (self.com[i] * self.mass + point[i] * point[3]) / newMass
		self.mass = newMass

		# Always increment contained as it is required for above checks
		self.contained[octantIndex] += 1

	def partition(self, octantIndex):
		# Copy points out of octants list
		points = self.octants[octantIndex]

		# Create octree
		newCenter, newWidth = self.findPartitionCenterWidth(octantIndex)

		self.octants[octantIndex] = octree(
			newCenter,
			newWidth,
			self.capacity,
			self.depth + 1,
			self.maxDepth
		)

		# Inserts all points into octree
		for point in points:
			self.octants[octantIndex].insert(point)

	def findPartitionCenterWidth(self, octantIndex):
		newWidth = self.width / 2
		newCenter = self.center.copy()

		# Calculating position of center of new octree within the octant
		# Refer to coordinate convention above
		if octantIndex < 4:
			newCenter[0] = newCenter[0] + newWidth
		else:
			newCenter[0] = newCenter[0] - newWidth

		if octantIndex in [0, 1, 4, 5]:
			newCenter[1] = newCenter[1] + newWidth
		else:
			newCenter[1] = newCenter[1] - newWidth

		if octantIndex % 2 == 0:
			newCenter[2] = newCenter[2] + newWidth
		else:
			newCenter[2] = newCenter[2] - newWidth

		return newCenter, newWidth

	def findOctant(self, point):
		index = 0

		# Refer to description above about axis conventions
		if point[0] <= self.center[0]:
			index += 4
		if point[1] <= self.center[1]:
			index += 2
		if point[2] <= self.center[2]:
			index += 1

		return index


def insertionBenchmark(n, coordinateRange, massRange, capacity):
	# Performs insertion of points to a new octree 5 times and reports time taken
	print(f'Benchmarking:	n: {n}	capacity: {capacity}')

	rng = np.random.default_rng()
	timeResults = np.zeros(5)

	for i in range(5):
		# Generates new points each time
		positions = rng.integers(-coordinateRange, coordinateRange, (n, 3))
		masses = rng.integers(1, massRange, (n, 1))
		points = np.hstack((positions, masses))
		width = (points.max() - points.min()) * 1.1
		center = [width / 2, width / 2, width / 2]

		# Tests time taken for a single run of insertion
		startTime = dt.datetime.now()
		pointsInsertion(center, width, capacity, points)
		endTime = dt.datetime.now()

		timeResults[i] = (endTime - startTime).microseconds / 10**6

	print(f'Seconds taken:	{timeResults.mean():.3f} ({timeResults.std():.3f})')
	print(f'Total seconds taken:	{timeResults.sum():.3f}')


def pointsInsertion(center, width, capacity, points):
	# Inserts all points

	newTree = octree(center, width, capacity)

	for i in range(points.shape[0]):
		newTree.insert(points[i])


if __name__ == '__main__':
	import numpy as np
	import datetime as dt

	n = 10**4
	coordinateRange = 10**5
	massRange = 10**3
	capacity = 1
	insertionBenchmark(n, coordinateRange, massRange, capacity)
