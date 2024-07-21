from odetest.rk import RungeKutta4
from odetest.rk import RungeKutta38Rule
import numpy as np
import pytest


class TestEuler:
    def test_RungeKutta4(self):
        # Integrates cos(t) from 0 to 0.5 * pi. Checks with tolerance of 1e-8.
        def d_func(t, y):
            return np.array([np.cos(t)])

        def func(t, y):
            return np.array([np.sin(t)])

        y0 = np.array([0.])
        h = 0.0001
        t_span = [0, 0.5 * np.pi]

        solver = RungeKutta4(d_func, t_span, y0, h)
        t, y, _iteration = solver.solve()

        expected = func(t_span[1], None)
        actual = y

        assert pytest.approx(expected, 1e-8) == actual

        # Integrates cos(t) + sin(t) from 0 to 10. Checks with tolerance of 1e-7.
        def d_func(t, y):
            return np.array([y[1], y[2], -y[0] - 3*y[1] - 3*y[2] - 4*np.sin(t)])

        def func(t, y):
            return np.array(np.cos(t) + np.sin(t))

        y0 = np.array([1., 1., -1.])
        h = 0.05
        t_span = [0, 10]

        solver = RungeKutta4(d_func, t_span, y0, h)
        t, y, _iteration = solver.solve()

        expected = func(t_span[1], None)
        actual = y[0]

        assert pytest.approx(expected, 1e-7) == actual

    def test_RungeKutta38Rule(self):
        # Integrates cos(t) from 0 to 0.5 * pi. Checks with tolerance of 1e-8.
        def d_func(t, y):
            return np.array([np.cos(t)])

        def func(t, y):
            return np.array([np.sin(t)])

        y0 = np.array([0.])
        h = 0.0001
        t_span = [0, 0.5 * np.pi]

        solver = RungeKutta38Rule(d_func, t_span, y0, h)
        t, y, _iteration = solver.solve()

        expected = func(t_span[1], None)
        actual = y

        assert pytest.approx(expected, 1e-8) == actual

        # Integrates cos(t) + sin(t) from 0 to 10. Checks with tolerance of 1e-7.
        def d_func(t, y):
            return np.array([y[1], y[2], -y[0] - 3*y[1] - 3*y[2] - 4*np.sin(t)])

        def func(t, y):
            return np.array(np.cos(t) + np.sin(t))

        y0 = np.array([1., 1., -1.])
        h = 0.05
        t_span = [0, 10]

        solver = RungeKutta38Rule(d_func, t_span, y0, h)
        t, y, _iteration = solver.solve()

        expected = func(t_span[1], None)
        actual = y[0]

        assert pytest.approx(expected, 1e-7) == actual
