package tree

import "github.com/snow259/Computational-Physics/src/vector"

type Quadtree struct {
	Center       vector.Vec2 // Coordinates of center of quadtree
	Dimensions   [2]float64  // Width and height
	CurrentDepth int
	MaxDepth     int
	Contents     []int       // Slice containing index of particles contained within the tree
	Mass         float64     // Mass of tree
	CoM          vector.Vec2 // Location of center of mass
}

func NewQuadtree(Center vector.Vec2, Dimensions [2]float64, MaxDepth int) *Quadtree {
	// Creates new Quadtree

	quadTree := Quadtree{}
	quadTree.Center = Center
	quadTree.Dimensions = Dimensions
	quadTree.CurrentDepth = 0
	quadTree.MaxDepth = MaxDepth

	return &quadTree
}

// Insert
// Find influencing points
// Walk
