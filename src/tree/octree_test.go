package tree

import (
	"github.com/snow259/Computational-Physics/src/vector"
	"testing"
)

func TestNewOctree(t *testing.T) {
	// Tests NewOctree() for creation of new Octree

	Center := vector.Vec3{0, 0, 0}
	Dimensions := [3]float64{10, 10, 10}
	MaxDepth := 5

	octree := NewOctree(Center, Dimensions, MaxDepth)

	if octree.Center != Center {
		t.Errorf("want %v, got %v", Center, octree.Center)
	}
	if octree.Dimensions != Dimensions {
		t.Errorf("want %v, got %v", Dimensions, octree.Dimensions)
	}
	if octree.MaxDepth != MaxDepth {
		t.Errorf("want %v, got %v", MaxDepth, octree.MaxDepth)
	}
	if octree.CurrentDepth != 0 {
		t.Errorf("want %v, got %v", 0, octree.CurrentDepth)
	}
}
