package vector

type Vector interface {
	// Mag methods are present in both Vec2 and Vec3
	// Vector interface for shared tests where possible

	Mag() float64
}
