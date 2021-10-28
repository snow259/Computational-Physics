import unittest
from octree import octree


class testOctree(unittest.TestCase):

	def test_findOctant(self):
		# Testing the function that determines the octant a point belongs to

		# Initialise tree
		center = [0, 0, 0]
		width = 10
		capacity = 10
		newTree = octree(center, width, capacity)

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
		center = [0, 0, 0]
		width = 10
		capacity = 10
		newTree = octree(center, width, capacity)

		# Expected results
		index0Result = ([5.0, 5.0, 5.0], 5.0)
		index1Result = ([5.0, 5.0, -5.0], 5.0)
		index2Result = ([5.0, -5.0, 5.0], 5.0)
		index3Result = ([5.0, -5.0, -5.0], 5.0)
		index4Result = ([-5.0, 5.0, 5.0], 5.0)
		index5Result = ([-5.0, 5.0, -5.0], 5.0)
		index6Result = ([-5.0, -5.0, 5.0], 5.0)
		index7Result = ([-5.0, -5.0, -5.0], 5.0)

		self.assertEqual(newTree.findPartitionCenterWidth(0), index0Result)
		self.assertEqual(newTree.findPartitionCenterWidth(1), index1Result)
		self.assertEqual(newTree.findPartitionCenterWidth(2), index2Result)
		self.assertEqual(newTree.findPartitionCenterWidth(3), index3Result)
		self.assertEqual(newTree.findPartitionCenterWidth(4), index4Result)
		self.assertEqual(newTree.findPartitionCenterWidth(5), index5Result)
		self.assertEqual(newTree.findPartitionCenterWidth(6), index6Result)
		self.assertEqual(newTree.findPartitionCenterWidth(7), index7Result)
