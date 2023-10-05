package tree

import "github.com/snow259/Computational-Physics/src/vector"

type Octree struct {
	Center       vector.Vec3
	Dimensions   [3]float64
	CurrentDepth int
	MaxDepth     int
}

func NewOctree(Center vector.Vec3, Dimensions [3]float64, MaxDepth int) *Octree {
	// Creates new root Octree

	octree := Octree{}
	octree.Center = Center
	octree.Dimensions = Dimensions
	octree.CurrentDepth = 0
	octree.MaxDepth = MaxDepth

	return &octree
}
