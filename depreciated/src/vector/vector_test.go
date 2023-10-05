package vector

import (
	"testing"
)

func TestInit(t *testing.T) {
	// Tests initialisation of Vector

	t.Run("Vec2 initialisation", func(t *testing.T) {
		var vector Vec2
		for _, point := range verificationPoints2() {
			vector = Vec2{point[0], point[1]}

			assertEqualFloat64(t, point[0], vector.X)
			assertEqualFloat64(t, point[1], vector.Y)
		}
	})

	t.Run("Vec3 initialisation", func(t *testing.T) {
		var vector Vec3
		for _, point := range verificationPoints3() {
			vector = Vec3{point[0], point[1], point[2]}

			assertEqualFloat64(t, point[0], vector.X)
			assertEqualFloat64(t, point[1], vector.Y)
			assertEqualFloat64(t, point[2], vector.Z)
		}
	})
}

func TestMag(t *testing.T) {
	// Tests Mag() return values for magnitude of Vector

	t.Run("Vec2 Mag", func(t *testing.T) {
		vectors := verificationVectors2()
		want := []float64{
			0.,
			4.123105625617661,
			1.4142135623730951,
			6.324555320336759,
			1.4142135623730951,
		}
		var got []float64

		for _, vector := range vectors {
			got = append(got, vector.Mag())
		}

		assertDeepEqualArr(t, want, got)
	})

	t.Run("Vec3 Mag", func(t *testing.T) {
		vectors := verificationVectors3()
		want := []float64{
			0.,
			4.242640687119285,
			1.7320508075688772,
			6.4031242374328485,
			5.196152422706632,
		}
		var got []float64

		for _, vector := range vectors {
			got = append(got, vector.Mag())
		}

		assertDeepEqualArr(t, want, got)
	})
}

func TestUnit(t *testing.T) {
	// Tests Unit() return values for unit vectors

	t.Run("Vec2 Unit", func(t *testing.T) {
		vectors := verificationVectors2()
		want := []Vector{
			Vec2{0, 0},
			Vec2{0.24253562503633297, 0.9701425001453319},
			Vec2{-0.7071067811865475, 0.7071067811865475},
			Vec2{0.9486832980505138, -0.31622776601683794},
			Vec2{-0.7071067811865475, -0.7071067811865475},
		}
		var got []Vector

		for _, vector := range vectors {
			got = append(got, vector.Unit())
		}

		assertDeepEqualVec(t, want, got)
	})

	t.Run("Vec3 Unit", func(t *testing.T) {
		vectors := verificationVectors3()
		want := []Vector{
			Vec3{0, 0, 0},
			Vec3{0.23570226039551587, 0.9428090415820635, 0.23570226039551587},
			Vec3{-0.5773502691896258, 0.5773502691896258, 0.5773502691896258},
			Vec3{0.9370425713316364, -0.31234752377721214, -0.15617376188860607},
			Vec3{-0.19245008972987526, -0.19245008972987526, -0.9622504486493763},
		}
		var got []Vector

		for _, vector := range vectors {
			got = append(got, vector.Unit())
		}

		assertDeepEqualVec(t, want, got)
	})
}

func TestScale(t *testing.T) {
	// Tests Scale() return values for scaled vectors

	t.Run("Vec2 Scale", func(t *testing.T) {
		vectors := verificationVectors2()
		want := []Vector{
			Vec2{0., 0.},
			Vec2{0.5, 2.},
			Vec2{0., 0},
			Vec2{-6., 2.},
			Vec2{0.5, 0.5},
		}
		scales := verificationScales()
		var got []Vector

		for i, vector := range vectors {
			got = append(got, vector.Scale(scales[i]))
		}

		assertDeepEqualVec(t, want, got)
	})

	t.Run("Vec3 Scale", func(t *testing.T) {
		vectors := verificationVectors3()
		want := []Vector{
			Vec3{0., 0., 0.},
			Vec3{0.5, 2., 0.5},
			Vec3{0., 0., 0.},
			Vec3{-6., 2., 1.},
			Vec3{0.5, 0.5, 2.5},
		}
		scales := verificationScales()
		var got []Vector

		for i, vector := range vectors {
			got = append(got, vector.Scale(scales[i]))
		}

		assertDeepEqualVec(t, want, got)
	})
}

func TestAdd(t *testing.T) {
	// Tests Add() for return values of added vectors

	t.Run("Vec2 Add", func(t *testing.T) {
		vectors := verificationVectors2()
		want := []Vector{
			Vec2{1., 4.},
			Vec2{0., 5.},
			Vec2{5., -1.},
			Vec2{5., -3.},
		}
		var got []Vector

		for i := range want {
			got = append(got, vectors[i].Add(vectors[i+1]))
		}

		assertDeepEqualVec(t, want, got)
	})

	t.Run("Vec3 Add", func(t *testing.T) {
		vectors := verificationVectors3()
		want := []Vector{
			Vec3{1., 4., 1.},
			Vec3{0., 5., 2.},
			Vec3{5., -1., 0.},
			Vec3{5., -3., -6.},
		}
		var got []Vector

		for i := range want {
			got = append(got, vectors[i].Add(vectors[i+1]))
		}

		assertDeepEqualVec(t, want, got)
	})
}

func TestSubtract(t *testing.T) {
	// Tests Subtract() for values of subtracted vectors

	t.Run("Vec2 Subtract", func(t *testing.T) {
		vectors := verificationVectors2()
		want := []Vector{
			Vec2{-1., -4.},
			Vec2{2., 3.},
			Vec2{-7., 3},
			Vec2{7., -1},
		}
		var got []Vector

		for i := range want {
			got = append(got, vectors[i].Subtract(vectors[i+1]))
		}

		assertDeepEqualVec(t, want, got)
	})

	t.Run("Vec3 Subtract", func(t *testing.T) {
		vectors := verificationVectors3()
		want := []Vector{
			Vec3{-1., -4., -1.},
			Vec3{2., 3., 0.},
			Vec3{-7., 3, 2.},
			Vec3{7., -1, 4.},
		}
		var got []Vector

		for i := range want {
			got = append(got, vectors[i].Subtract(vectors[i+1]))
		}

		assertDeepEqualVec(t, want, got)
	})
}

func TestDot(t *testing.T) {
	// Tests Dot() for return values of dot product

	t.Run("Vec2 Dot", func(t *testing.T) {
		vectors := verificationVectors2()
		want := []float64{0., 3., -8., -4.}
		var got []float64

		for i := range want {
			got = append(got, vectors[i].Dot(vectors[i+1]))
		}

		assertDeepEqualArr(t, want, got)
	})

	t.Run("Vec3 Dot", func(t *testing.T) {
		vectors := verificationVectors3()
		want := []float64{0., 4., -9., 1.}
		var got []float64

		for i := range want {
			got = append(got, vectors[i].Dot(vectors[i+1]))
		}

		assertDeepEqualArr(t, want, got)
	})
}

func TestCross(t *testing.T) {
	// Tests Cross() for return values of cross product

	t.Run("Vec2 Cross", func(t *testing.T) {
		vectors := verificationVectors2()
		want := []float64{0., 5., -4., -8.}
		var got []float64

		for i := range want {
			got = append(got, vectors[i].Cross(vectors[i+1]))
		}

		assertDeepEqualArr(t, want, got)
	})

	t.Run("Vec3 Cross", func(t *testing.T) {
		vectors := verificationVectors3()
		want := []Vector{
			Vec3{0., 0., 0.},
			Vec3{3., -2., 5.},
			Vec3{1., 5., -4.},
			Vec3{9., 31., -8.},
		}
		var got []Vector

		for i := range want {
			got = append(got, vectors[i].Cross(vectors[i+1]))
		}

		assertDeepEqualVec(t, want, got)
	})
}
