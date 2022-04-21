package vector

import (
	"fmt"
	"math"
)

type Vec2 struct {
	X float64
	Y float64
}

func (u Vec2) String() string {
	// String format for printing

	return fmt.Sprintf("(%v, %v)", u.X, u.Y)
}

func (u Vec2) Mag() float64 {
	// Returns magnitude of vector

	return math.Sqrt(u.X*u.X + u.Y*u.Y)
}

func (u Vec2) Unit() Vec2 {
	// Returns unit vector

	mag := u.Mag()

	if mag == 0.0 {
		return Vec2{0, 0}
	}

	return Vec2{u.X / mag, u.Y / mag}
}

func (u Vec2) Scale(k float64) Vec2 {
	// Scales vector u by constant k

	return Vec2{u.X * k, u.Y * k}
}

func (u Vec2) Add(v Vec2) Vec2 {
	// Returns vector sum of u and v

	return Vec2{u.X + v.X, u.Y + v.Y}
}

func (u Vec2) Subtract(v Vec2) Vec2 {
	// Returns vector subtraction of u and v

	return Vec2{u.X - v.X, u.Y - v.Y}
}

func (u Vec2) Dot(v Vec2) float64 {
	// Returns dot product of vectors u and v

	return u.X*v.X + u.Y*v.Y
}
func (u Vec2) Cross(v Vec2) float64 {
	// Returns cross product of vectors u and v
	// Cross product in two dimensions does not make much sense, thus it can be assumed to be two three-dimensional
	// vectors with Z component set to 0.
	// Returns magnitude of Z component of a Vec3 if both Vec2 inputs were moved to Vec3

	return u.X*v.Y - v.X*u.Y
}
