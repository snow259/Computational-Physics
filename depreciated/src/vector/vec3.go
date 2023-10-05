package vector

import (
	"fmt"
	"math"
)

type Vec3 struct {
	X float64
	Y float64
	Z float64
}

func (u Vec3) String() string {
	// String format for printing

	return fmt.Sprintf("(%v, %v, %v)", u.X, u.Y, u.Z)
}

func (u Vec3) Mag() float64 {
	// Returns magnitude of vector

	return math.Sqrt(u.X*u.X + u.Y*u.Y + u.Z*u.Z)
}

func (u Vec3) Unit() Vec3 {
	// Returns unit vector of u

	mag := u.Mag()

	if mag == 0.0 {
		return Vec3{0, 0, 0}
	}

	return Vec3{u.X / mag, u.Y / mag, u.Z / mag}
}

func (u Vec3) Scale(k float64) Vec3 {
	// Scales vector u by constant k

	return Vec3{u.X * k, u.Y * k, u.Z * k}
}

func (u Vec3) Add(v Vec3) Vec3 {
	// Returns vector sum of u and v

	return Vec3{u.X + v.X, u.Y + v.Y, u.Z + v.Z}
}

func (u Vec3) Subtract(v Vec3) Vec3 {
	// Returns vector subtraction of u and v

	return Vec3{u.X - v.X, u.Y - v.Y, u.Z - v.Z}
}

func (u Vec3) Dot(v Vec3) float64 {
	// Returns dot product of vectors u and v

	return u.X*v.X + u.Y*v.Y + u.Z*v.Z
}

func (u Vec3) Cross(v Vec3) Vec3 {
	// Returns cross product of vectors u and v

	X := u.Y*v.Z - v.Y*u.Z
	Y := u.Z*v.X - v.Z*u.X
	Z := u.X*v.Y - v.X*u.Y

	return Vec3{X, Y, Z}
}
