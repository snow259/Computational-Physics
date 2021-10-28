class octree:

	def __init__(self, center, width, capacity):
		# Define position and size
		self.center = center
		self.width = width

		# Define capacity of each octant
		self.capacity = capacity

		# Mass contained in octant, and position of center of mass
		self.mass = 0
		self.com = center

		# Contents of octants
		# Right handed axis, x towards, y right, z upwards
		# 0: +++ : fTopRight
		# 1: ++- : fTopLeft
		# 2: +-+ : fBotRight
		# 3: +-- : fBotLeft
		# 4: -++ : rTopRight
		# 5: -+- : rTopLeft
		# 6: --+ : rBotRight
		# 7: --- : rBotLeft
		# 8 nested lists, because initialising with [] * 8 will create 8 references
		# to the same object in memory
		self.octants = [[], [], [], [], [], [], [], []]
		self.contained = [0] * 8

	def insert(self, point):
		# Find octant to insertn point, check octant for capicity
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
		# Copy points out of octants list
		points = self.octants[octantIndex]

		# Create octree
		newCenter, newWidth = self.findPartitionCenterWidth(octantIndex)
		self.octants[octantIndex] = octree(newCenter, newWidth, self.capacity)
		# Inserts all points into octree
		for point in points:
			self.octants[octantIndex].insert(point)

	def findPartitionCenterWidth(self, octantIndex):
		newWidth = self.width / 2
		newCenter = self.center

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
