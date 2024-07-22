from odetest.rk import RungeKutta4
from odetest.rk import RungeKutta38Rule
import numpy as np
import pytest


test1 = {
    "fun": lambda t, y: np.array(np.cos(t)),
    "t_span": [0, 0.5 * np.pi],
    "y0": np.array([0.]),
    "h": 0.0001,
    "tolerance": 1e-4,
    "expected": lambda t, y: np.array([np.sin(t)]),
}
test2 = {
    "fun": lambda t, y: np.array([y[1], y[2], -y[0] - 3 * y[1] - 3 * y[2] - 4 * np.sin(t)]),
    "t_span": [0, 10],
    "y0": np.array([1., 1., -1.]),
    "h": 0.01,
    "tolerance": 1e-2,
    "expected": lambda t, y: np.array([np.cos(t) + np.sin(t)]),
}

@pytest.mark.parametrize("test_input", [(test1), (test2)])
class TestRungeKutta:
    def test_RungeKutta4(self, test_input):
        fun = test_input["fun"]
        t_span = test_input["t_span"]
        y0 = test_input["y0"]
        h = test_input["h"]
        tolerance = test_input["tolerance"]
        expected = test_input["expected"](t_span[1], None)[0]

        solver = RungeKutta4(fun, t_span, y0, h)
        _t, actual, _iteration = solver.solve()

        assert pytest.approx(expected, tolerance) == actual[0]

    def test_RungeKutta38Rule(self, test_input):
        fun = test_input["fun"]
        t_span = test_input["t_span"]
        y0 = test_input["y0"]
        h = test_input["h"]
        tolerance = test_input["tolerance"]
        expected = test_input["expected"](t_span[1], None)[0]

        solver = RungeKutta38Rule(fun, t_span, y0, h)
        _t, actual, _iteration = solver.solve()

        assert pytest.approx(expected, tolerance) == actual[0]