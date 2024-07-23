from odetest.vector import Vector2


class TestVector:
    def test_add_and_dot(self):
        # Add a vector to itself and multiply a scalar value of 2 to it and compare both operations, should be equal
        vec1 = Vector2(-1, 2)
        vec2 = vec1 + vec1

        assert vec2 == 2 * vec1

    def test_subtract(self):
        # Add a vector to itself and then subtract that vector from it, value should be unchanged
        vec1 = Vector2(-1, 2)
        vec2 = vec1 + vec1 - vec1

        assert vec2 == vec1

    def test_mag(self):
        # Compare two times magnitude of vector to magnitude of the product of same vector with two, should be equal
        vec1 = Vector2(1, 1)

        assert 2 * (vec1.mag()) == (2 * vec1).mag()
