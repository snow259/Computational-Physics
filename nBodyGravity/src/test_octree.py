import unittest
from octree import octree


class testOctree(unittest.TestCase):

	def verificationPoints(self):
		# Returns dict containing all test points used in the unittests
		# Initially named testPoints but that gets counted as a test function

		# Right handed axis, x right, y upwards, z towards
		# 0: +++ : right, top, front
		# 1: ++- : right, top, rear
		# 2: +-+ : right, bot, front
		# 3: +-- : right, bot, rear
		# 4: -++ : left, top, front
		# 5: -+- : left, top, rear
		# 6: --+ : left, bot, front
		# 7: --- : left, bot, rear
		points = {
			'pointRTF': [1, 1, 1, 10],
			'pointRTR': [1, 1, -1, 10],
			'pointRBF': [1, -1, 1, 10],
			'pointRBR': [1, -1, -1, 10],
			'pointLTF': [-1, 1, 1, 10],
			'pointLTR': [-1, 1, -1, 10],
			'pointLBF': [-1, -1, 1, 10],
			'pointLBR': [-1, -1, -1, 10],
			'pointCenter': [0, 0, 0, 10],
			'onePartitionRTF': [6, 6, 6, 10],
			'twoPartitionLBR': [-4, -4, -4, 10],
			'pointCom': [0, 0, 0]
		}

		return points

	def emptyTree(self, center=[0, 0, 0], width=10, capacity=10):
		# Returns an empty tree
		newTree = octree(center, width, capacity)

		return newTree

	def partiallyFilledTree(self, capacity=2):
		# Returns tree with some points already inserted, but no partitions
		newTree = self.emptyTree(capacity=capacity)
		points = self.verificationPoints()

		# Points with 0 to 3 negative coordinates, and edge case of center
		newTree.insert(points['pointRTF'])
		newTree.insert(points['pointRBF'])
		newTree.insert(points['pointLTR'])
		if capacity > 1:
			# Both this point and pointCenter will be inserted to octant 7
			newTree.insert(points['pointLBR'])
		newTree.insert(points['pointCenter'])

		return newTree

	def test_findOctant(self):
		# Testing the function that determines the octant a point belongs to

		# Initialise tree and points
		newTree = self.emptyTree()
		points = self.verificationPoints()

		# Testing with full octant coverage, and an edge case of point in center
		self.assertEqual(newTree.findOctant(points['pointRTF']), 0)
		self.assertEqual(newTree.findOctant(points['pointRTR']), 1)
		self.assertEqual(newTree.findOctant(points['pointRBF']), 2)
		self.assertEqual(newTree.findOctant(points['pointRBR']), 3)
		self.assertEqual(newTree.findOctant(points['pointLTF']), 4)
		self.assertEqual(newTree.findOctant(points['pointLTR']), 5)
		self.assertEqual(newTree.findOctant(points['pointLBF']), 6)
		self.assertEqual(newTree.findOctant(points['pointLBR']), 7)
		self.assertEqual(newTree.findOctant(points['pointCenter']), 7)

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
		points = self.verificationPoints()

		# Test insertion with 0 to 2 negative coords, and edge case of center
		# 3 negative coords not tested  here
		# Capacity is 1, and both it and center will go to octantIndex 7
		newTree.insert(points['pointRTF'])
		newTree.insert(points['pointRBF'])
		newTree.insert(points['pointLTR'])
		newTree.insert(points['pointCenter'])

		# Test position of stored points in octree
		self.assertEqual(newTree.octants[0], [points['pointRTF']])
		self.assertEqual(newTree.octants[2], [points['pointRBF']])
		self.assertEqual(newTree.octants[5], [points['pointLTR']])
		self.assertEqual(newTree.octants[7], [points['pointCenter']])

		#  Initialise tree with capacity 10
		newTree = self.emptyTree(capacity=10)

		# Insert all points as there is capacity
		newTree.insert(points['pointRTF'])
		newTree.insert(points['pointRBF'])
		newTree.insert(points['pointLTR'])
		newTree.insert(points['pointLBR'])
		newTree.insert(points['pointCenter'])

		# Test position of stored points in octree
		self.assertEqual(newTree.octants[0], [points['pointRTF']])
		self.assertEqual(newTree.octants[2], [points['pointRBF']])
		self.assertEqual(newTree.octants[5], [points['pointLTR']])
		self.assertEqual(newTree.octants[7], [points['pointLBR'], points['pointCenter']])

	def test_partition(self):
		# Tests partitioning of octants by inducing single and double partitions
		# Compares values of depth, coordinates, and mass

		# Test with capacity 1
		newTree = self.partiallyFilledTree(capacity=1)
		points = self.verificationPoints()

		# Insert points to induce partitions
		newTree.insert(points['onePartitionRTF'])
		newTree.insert(points['twoPartitionLBR'])

		# Single partition tests
		# Test position of stored points in octree
		self.assertEqual(newTree.octants[0].octants[0], [points['onePartitionRTF']])
		self.assertEqual(newTree.octants[0].octants[7], [points['pointRTF']])

		# Test depth
		self.assertEqual(newTree.octants[0].depth, 1)

		# Double partition tests
		# Test position of stored points in octree
		self.assertEqual(newTree.octants[7].octants[0].octants[0], [points['pointCenter']])
		self.assertEqual(newTree.octants[7].octants[0].octants[7], [points['twoPartitionLBR']])

		# Test with capacity 2, only testing double partition here
		newTree = self.partiallyFilledTree(capacity=2)
		newTree.insert(points['twoPartitionLBR'])

		# Double partition tests
		# Test position of stored points in octree
		self.assertEqual(
			newTree.octants[7].octants[0].octants[0], [points['pointLBR'], points['pointCenter']]
		)
		self.assertEqual(newTree.octants[7].octants[0].octants[7], [points['twoPartitionLBR']])

		# Test depth
		self.assertEqual(newTree.octants[7].octants[0].depth, 2)

	def findCom(self, points):
		# Finds the center of mass of a given list of points
		com = [0, 0, 0]
		mass = 0
		for point in points:
			mass += point[3]
			for i in range(len(com)):
				com[i] = com[i] + point[i] * point[3]

		for i in range(len(com)):
			com[i] = com[i] / mass

		return com

	def test_computeMassDist(self):
		# Tests the compuation of mass and com

		points = self.verificationPoints()

		# Center of masses for the three com tests in order of testing
		# Center of mass for single partition test
		com1 = self.findCom([
			points['onePartitionRTF'],
			points['pointRTF']
		])
		# Center of mass for double partition test
		com2 = self.findCom([
			points['pointCenter'],
			points['twoPartitionLBR'],
			points['pointLBR']
		])
		# Center of mass of all points in double partition test
		com2Total = self.findCom([
			points['pointRTF'],
			points['pointRBF'],
			points['pointLTR'],
			points['pointLBR'],
			points['pointCenter'],
			points['twoPartitionLBR']
		])

		# Test with capacity 1, testing only single partition here
		newTree = self.partiallyFilledTree(capacity=1)
		newTree.insert(points['onePartitionRTF'])
		newTree.computeMassDist()

		# Test mass
		self.assertAlmostEqual(newTree.com[3], 50)
		self.assertAlmostEqual(newTree.octants[0].com[3], 20)
		self.assertAlmostEqual(newTree.octants[0].com[0], com1[0])
		self.assertAlmostEqual(newTree.octants[0].com[1], com1[1])
		self.assertAlmostEqual(newTree.octants[0].com[2], com1[2])

		# Test with capacity 2, only testing double partition here
		newTree = self.partiallyFilledTree(capacity=2)
		newTree.insert(points['twoPartitionLBR'])
		newTree.computeMassDist()

		# Test mass and com of left bot rear octant
		# Initially has three points, so it is paritioned once
		# First parititon has three points so it is partitioned a second time
		# Second partition has 1 point in an octant, and 2 in another
		self.assertAlmostEqual(newTree.com[3], 60)
		self.assertAlmostEqual(newTree.com[0], com2Total[0])
		self.assertAlmostEqual(newTree.com[1], com2Total[1])
		self.assertAlmostEqual(newTree.com[2], com2Total[2])
		self.assertAlmostEqual(newTree.octants[7].com[3], 30)
		self.assertAlmostEqual(newTree.octants[7].octants[0].com[3], 30)
		self.assertAlmostEqual(newTree.octants[7].octants[0].com[0], com2[0])
		self.assertAlmostEqual(newTree.octants[7].octants[0].com[1], com2[1])
		self.assertAlmostEqual(newTree.octants[7].octants[0].com[2], com2[2])

	def test_distance(self):
		# Tests return value of distance function

		# Tree properties or contained points are not required for this function
		newTree = self.emptyTree()
		points = self.verificationPoints()

		self.assertAlmostEqual(newTree.distance(points['onePartitionRTF'], points['pointCenter']), 6 * 3**0.5)
		self.assertAlmostEqual(newTree.distance(points['pointLBR'], points['pointCom']), 1 * 3**0.5)
		self.assertAlmostEqual(newTree.distance(points['pointRTF'], points['pointRTR']), 2)
		self.assertAlmostEqual(newTree.distance(points['pointLBR'], points['pointRTF']), 2 * 3**0.5)


if __name__ == '__main__':
	# Enable unit testing from within editor
	# Throws errors when using Sublime REPL to build, but completes testing
	unittest.main()
