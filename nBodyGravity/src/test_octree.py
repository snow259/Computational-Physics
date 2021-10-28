import unittest
from octree import octree


class testOctree(unittest.TestCase):

	def emptyTree(self, center=[0, 0, 0], width=10, capacity=10):
		# Returns an empty tree
		newTree = octree(center, width, capacity)

		return newTree

	def test_findOctant(self):
		# Testing the function that determines the octant a point belongs to

		# Initialise tree
		center = [0, 0, 0]
		newTree = self.emptyTree()

		# Test Points
		# Right handed axis, x right, y upwards, z towards
		# 0: +++ : right, top, front
		# 1: ++- : right, top, rear
		# 2: +-+ : right, bot, front
		# 3: +-- : right, bot, rear
		# 4: -++ : left, top, front
		# 5: -+- : left, top, rear
		# 6: --+ : left, bot, front
		# 7: --- : left, bot, rear
		pointRTF = [1, 1, 1, 10]
		pointRTR = [1, 1, -1, 10]
		pointRBF = [1, -1, 1, 10]
		pointRBR = [1, -1, -1, 10]
		pointLTF = [-1, 1, 1, 10]
		pointLTR = [-1, 1, -1, 10]
		pointLBF = [-1, -1, 1, 10]
		pointLBR = [-1, -1, -1, 10]
		pointCenter = center

		# Testing with full octant coverage, and an edge case of point in center
		self.assertEqual(newTree.findOctant(pointRTF), 0)
		self.assertEqual(newTree.findOctant(pointRTR), 1)
		self.assertEqual(newTree.findOctant(pointRBF), 2)
		self.assertEqual(newTree.findOctant(pointRBR), 3)
		self.assertEqual(newTree.findOctant(pointLTF), 4)
		self.assertEqual(newTree.findOctant(pointLTR), 5)
		self.assertEqual(newTree.findOctant(pointLBF), 6)
		self.assertEqual(newTree.findOctant(pointLBR), 7)
		self.assertEqual(newTree.findOctant(pointCenter), 7)

	def test_findPartitionCenterWidth(self):
		# Testing function that finds the position and width of new octree

		# Initialise tree
		newTree = self.emptyTree()

		# Expected results
		index0Result = ([5.0, 5.0, 5.0], 5.0)
		index1Result = ([5.0, 5.0, -5.0], 5.0)
		index2Result = ([5.0, -5.0, 5.0], 5.0)
		index3Result = ([5.0, -5.0, -5.0], 5.0)
		index4Result = ([-5.0, 5.0, 5.0], 5.0)
		index5Result = ([-5.0, 5.0, -5.0], 5.0)
		index6Result = ([-5.0, -5.0, 5.0], 5.0)
		index7Result = ([-5.0, -5.0, -5.0], 5.0)

		# Testing for full coverage
		# These 8 values are the only possible return values from findOctant()
		self.assertEqual(newTree.findPartitionCenterWidth(0), index0Result)
		self.assertEqual(newTree.findPartitionCenterWidth(1), index1Result)
		self.assertEqual(newTree.findPartitionCenterWidth(2), index2Result)
		self.assertEqual(newTree.findPartitionCenterWidth(3), index3Result)
		self.assertEqual(newTree.findPartitionCenterWidth(4), index4Result)
		self.assertEqual(newTree.findPartitionCenterWidth(5), index5Result)
		self.assertEqual(newTree.findPartitionCenterWidth(6), index6Result)
		self.assertEqual(newTree.findPartitionCenterWidth(7), index7Result)

	def test_insert(self):
		# Tests insertion of points and their stored data

		# Initialise tree with capacity 1
		newTree = self.emptyTree(capacity=1)

		# Points with 0 to 3 negative coordinates, and edge case of center
		pointRTF = [1, 1, 1, 10]
		pointRBF = [1, -1, 1, 10]
		pointLTR = [-1, 1, -1, 10]
		pointLBR = [-1, -1, -1, 10]
		pointCenter = [0, 0, 0, 10]

		# Test insertion with 0 to 2 negative coords, and edge case of center
		# 3 negative coords not tested  here
		# Capacity is 1, and both it and center will go to octantIndex 7
		newTree.insert(pointRTF)
		newTree.insert(pointRBF)
		newTree.insert(pointLTR)
		newTree.insert(pointCenter)

		# Test values
		self.assertEqual(newTree.octants[0], [pointRTF])
		self.assertEqual(newTree.octants[2], [pointRBF])
		self.assertEqual(newTree.octants[5], [pointLTR])
		self.assertEqual(newTree.octants[7], [pointCenter])

		#  Initialise tree with capacity 10
		newTree = self.emptyTree(capacity=10)

		# Insert all points as there is capacity
		newTree.insert(pointRTF)
		newTree.insert(pointRBF)
		newTree.insert(pointLTR)
		newTree.insert(pointLBR)
		newTree.insert(pointCenter)

		# Test values
		self.assertEqual(newTree.octants[0], [pointRTF])
		self.assertEqual(newTree.octants[2], [pointRBF])
		self.assertEqual(newTree.octants[5], [pointLTR])
		self.assertEqual(newTree.octants[7], [pointLBR, pointCenter])

	def partiallyFilledTree(self, capacity=2):
		# Returns tree with some points already inserted, but no partitions
		newTree = self.emptyTree(capacity=capacity)

		# Points with 0 to 3 negative coordinates, and edge case of center
		pointRTF = [1, 1, 1, 10]
		pointRBF = [1, -1, 1, 10]
		pointLTR = [-1, 1, -1, 10]
		pointLBR = [-1, -1, -1, 10]
		pointCenter = [0, 0, 0, 10]

		newTree.insert(pointRTF)
		newTree.insert(pointRBF)
		newTree.insert(pointLTR)
		if capacity > 1:
			# Both this point and pointCenter will be inserted to octant 7
			newTree.insert(pointLBR)
		newTree.insert(pointCenter)

		return newTree

	def test_partition(self):
		# Tests partitioning of octants by inducing single and double partitions
		# Compares values of depth, coordinates, and mass

		# Test with capacity 1
		newTree = self.partiallyFilledTree(capacity=1)

		pointRTF = [1, 1, 1, 10]
		pointLBR = [-1, -1, -1, 10]
		pointCenter = [0, 0, 0, 10]

		# Points to induce partitions
		onePartitionRTF = [6, 6, 6, 10]
		twoPartitionLBR = [-4, -4, -4, 10]

		newTree.insert(onePartitionRTF)
		newTree.insert(twoPartitionLBR)

		# Single partition tests
		self.assertEqual(newTree.octants[0].octants[0], [onePartitionRTF])
		self.assertEqual(newTree.octants[0].octants[7], [pointRTF])
		self.assertEqual(newTree.octants[0].depth, 1)

		# Double partition tests
		self.assertEqual(newTree.octants[7].octants[0].octants[0], [pointCenter])
		self.assertEqual(newTree.octants[7].octants[0].octants[7], [twoPartitionLBR])

		# Test with capacity 2, only testing double partition here
		newTree = self.partiallyFilledTree(capacity=2)
		newTree.insert(twoPartitionLBR)

		# Double partition tests
		self.assertEqual(
			newTree.octants[7].octants[0].octants[0], [pointLBR, pointCenter]
		)
		self.assertEqual(newTree.octants[7].octants[0].octants[7], [twoPartitionLBR])
		self.assertEqual(newTree.octants[7].octants[0].depth, 2)


if __name__ == '__main__':
	# Enable unit testing from within editor
	# Throws errors when using Sublime REPL to build, but completes testing
	unittest.main()
