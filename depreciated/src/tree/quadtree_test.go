package tree

import (
	"github.com/snow259/Computational-Physics/src/vector"
	"testing"
)

func TestNewQuadtree(t *testing.T) {
	// Tests NewQuadtree() for creation of new Octree

	Center := vector.Vec2{0, 0}
	Dimensions := [2]float64{10, 10}
	MaxDepth := 5

	quadtree := NewQuadtree(Center, Dimensions, MaxDepth)

	if quadtree.Center != Center {
		t.Errorf("want %v, got %v", Center, quadtree.Center)
	}
	if quadtree.Dimensions != Dimensions {
		t.Errorf("want %v, got %v", Dimensions, quadtree.Dimensions)
	}
	if quadtree.MaxDepth != MaxDepth {
		t.Errorf("want %v, got %v", MaxDepth, quadtree.MaxDepth)
	}
	if quadtree.CurrentDepth != 0 {
		t.Errorf("want %v, got %v", 0, quadtree.CurrentDepth)
	}
}
