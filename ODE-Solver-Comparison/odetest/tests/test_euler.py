from odetest.euler import Euler
import numpy as np
import pytest


class TestEuler:
    def test_solve(self):
        # Integrates cos(x) from 0 to 0.5 * pi. Checks with tolerance of 1e-4.
        def d_func(t, y):
            return np.array(np.cos(t))

        def func(t, y):
            return np.array(np.sin(t))

        y0 = np.array(0.)
        h = 0.0001
        t_span = [0, 0.5 * np.pi]

        solver = Euler(d_func, t_span, y0, h)
        t, y, _iteration = solver.solve()

        expected = func(t_span[1], None)
        actual = y

        assert pytest.approx(expected, 1e-4) == actual
        # assert t <= t_span[1]

        # Integrates cos(t) + sin(t) from 0 to 10. Checks with tolerance of 1e-2.
        def d_func(t, y):
            return np.array([y[1], y[2], -y[0] - 3*y[1] - 3*y[2] - 4*np.sin(t)])

        def func(t, y):
            return np.array(np.cos(t) + np.sin(t))

        y0 = np.array([1., 1., -1.])
        h = 0.01
        t_span = [0, 10]

        solver = Euler(d_func, t_span, y0, h)
        t, y, _iteration = solver.solve()

        expected = func(t_span[1], None)
        actual = y[0]

        assert pytest.approx(expected, 1e-2) == actual
