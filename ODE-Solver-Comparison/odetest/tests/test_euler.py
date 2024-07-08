from odetest.euler import Euler
import math
import pytest


class TestEuler:
    def test_solve(self):
        # Integrates cos(x) from 0 to 0.5 * pi. Checks with tolerance of 1e-4.
        def d_func(t, y):
            return [math.cos(t)]

        def func(t, y):
            return [math.sin(t)]

        solver = Euler(d_func, [0, 0.5 * math.pi], [0], 0.0001)
        t, y, _iteration = solver.solve()

        expected = func(0.5 * math.pi, None)[0]
        actual = y[0]

        assert pytest.approx(expected, 1e-4) == actual
