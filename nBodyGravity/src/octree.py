import numpy as np


class octree:

	def __init__(self, center, width, capacity, depth=0, maxDepth=10):
		# Define position and size
		self.center = center
		self.width = width

		# Define capacity of each octant
		self.capacity = capacity

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
		# Inserts points to octants

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

		# Always increment contained as it is required for above checks
		self.contained[octantIndex] += 1

	def partition(self, octantIndex):
		# Partitions octants if over capacity after insertion

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
		# Finds the center and width of a new child octant based on index
		# and current octant center and width

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
		# Given coordinates and current octree center and width,
		# finds octant point belongs to

		index = 0

		# Refer to description above about axis conventions
		if point[0] <= self.center[0]:
			index += 4
		if point[1] <= self.center[1]:
			index += 2
		if point[2] <= self.center[2]:
			index += 1

		return index

	def computeMassDist(self):
		# Computes mass and com for entire tree
		# Stores com and mass in a single array like, similar to points

		# com = [0, 0, 0]
		com = np.zeros(4)
		for i in range(8):
			if self.contained[i] <= self.capacity:
				# Octant has only points, add their masses and compute com
				for point in self.octants[i]:
					com[3] += point[3]
					for j in range(3):
						com[j] += point[j] * point[3]
			else:
				# Octant contains an octree instead, query them for mass and com
				self.octants[i].computeMassDist()
				com[3] += self.octants[i].com[3]
				for j in range(3):
					com[j] += self.octants[i].com[j] * self.octants[i].com[3]

		for i in range(3):
			com[i] = com[i] / com[3]

		self.com = com

	def findContributingMasses(self, point, theta):
		# Finds all masses that contribute force to point, for Barnes Hut algorithm

		contributingMasses = list()
		for i in range(8):
			if self.contained[i] <= self.capacity:
				# Octant contains only points
				for point in self.octants[i]:
					if point is not None:
						contributingMasses.append(point)
			else:
				# Octant contains octree
				com = self.octants[i].com.copy()
				r = self.distance(point, self.octants[i].com)
				d = self.octants[i].width * 2
				if d / r < theta:
					# Approximation acceptable
					contributingMasses.append(com)
				else:
					# Approximation not acceptable, collect points from octant
					contributingMasses.extend(self.octants[i].findContributingMasses(point, theta))

		return contributingMasses

	def distance(self, point1, point2):
		# Returns distance between points

		dist = 0
		for i in range(3):
			dist += (point2[i] - point1[i]) * (point2[i] - point1[i])

		return dist**0.5


def insertionBenchmark(n, coordinateRange, massRange, capacity, theta, runs):
	# Performs insertion of points to a new octree 5 times and reports time taken
	print(f'\nBenchmarking:	n: {n}	capacity: {capacity}	theta: {theta}	runs: {runs}\n')

	rng = np.random.default_rng()
	treeBuildResults = np.zeros(runs)
	searchContributorResults = np.zeros(runs)

	for i in range(runs):
		# Generates new points each time
		positions = rng.integers(-coordinateRange, coordinateRange, (n, 3))
		masses = rng.integers(1, massRange, (n, 1))
		points = np.hstack((positions, masses))
		width = (points.max() - points.min()) * 1.1
		center = [width / 2, width / 2, width / 2]

		# Tests time taken for a single run of insertion
		startTime = dt.datetime.now()
		newTree = pointsInsertion(center, width, capacity, points)
		endTime = dt.datetime.now()
		treeBuildResults[i] = (endTime - startTime).total_seconds()

		startTime = dt.datetime.now()
		contributorSearch(newTree, theta, points)
		endTime = dt.datetime.now()
		searchContributorResults[i] = (endTime - startTime).total_seconds()

	print('Building Tree:')
	print(f'Seconds taken:	{treeBuildResults.mean():.3f} ({treeBuildResults.std():.3f})')
	print(f'Total seconds taken:	{treeBuildResults.sum():.3f}\n')
	print('Searching all contributing masses:')
	print(f'Seconds taken:	{searchContributorResults.mean():.3f} ({searchContributorResults.std():.3f})')
	print(f'Total seconds taken:	{searchContributorResults.sum():.3f}')


def pointsInsertion(center, width, capacity, points):
	# Inserts all points

	newTree = octree(center, width, capacity)

	# Insert points
	for i in range(points.shape[0]):
		newTree.insert(points[i])

	# Mass distribution computation
	newTree.computeMassDist()

	return newTree


def contributorSearch(newTree, theta, points):
	for point in points:
		_ = newTree.findContributingMasses(point, theta)


if __name__ == '__main__':
	import datetime as dt

	n = 10**4
	coordinateRange = 10**5
	massRange = 10**3
	capacity = 1
	theta = 0.5
	runs = 5
	insertionBenchmark(n, coordinateRange, massRange, capacity, theta, runs)
