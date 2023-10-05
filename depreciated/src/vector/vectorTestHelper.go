package vector

import (
	"reflect"
	"testing"
)

func verificationPoints2() [5][2]float64 {
	// Returns an array of points for testing Vec2 creation

	var points [5][2]float64

	points[0] = [2]float64{0., 0.}
	points[1] = [2]float64{1., 4.}
	points[2] = [2]float64{-1., 1.}
	points[3] = [2]float64{6., -2.}
	points[4] = [2]float64{-1., -1.}

	return points
}

func verificationPoints3() [5][3]float64 {
	// Returns an array of points for testing Vec3 creation

	var points [5][3]float64

	points[0] = [3]float64{0., 0., 0.}
	points[1] = [3]float64{1., 4., 1.}
	points[2] = [3]float64{-1., 1., 1.}
	points[3] = [3]float64{6., -2., -1.}
	points[4] = [3]float64{-1., -1., -5.}

	return points
}

func verificationVectors2() [5]Vec2 {
	// Returns an array of Vec2 from verificationPoints2()

	var vectors [5]Vec2
	points := verificationPoints2()

	for i, point := range points {
		vectors[i] = Vec2{point[0], point[1]}
	}

	return vectors
}

func verificationVectors3() [5]Vec3 {
	// Returns an array of Vec3 from verificationPoints3()

	var vectors [5]Vec3
	points := verificationPoints3()

	for i, point := range points {
		vectors[i] = Vec3{point[0], point[1], point[2]}
	}

	return vectors
}

func verificationScales() [5]float64 {
	// Retuns an array of floats for testing Scale()

	scales := [5]float64{
		1,
		0.5,
		0,
		-1,
		-0.5,
	}

	return scales
}

func assertEqualFloat64(t testing.TB, want float64, got float64) {
	// Asserts equality of two float64

	t.Helper()

	if want != got {
		t.Errorf("want %v, got %v", want, got)
	}
}

func assertDeepEqualArr(t testing.TB, want []float64, got []float64) {
	// Asserts deep equal of two []float64

	t.Helper()

	if !reflect.DeepEqual(want, got) {
		t.Errorf("want %v, got %v", want, got)
	}
}

func assertDeepEqualVec(t testing.TB, want []Vector, got []Vector) {
	// Asserts deep equal of two []Vector

	t.Helper()

	if !reflect.DeepEqual(want, got) {
		t.Errorf("want %v, got %v", want, got)
	}
}
