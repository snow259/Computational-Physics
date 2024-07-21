from .solver import Solver
import numpy as np


class RungeKutta(Solver):
    def step(self):
        Q = np.zeros(self.n_stages)
        self.t_prev = self.t

        # Checks to see if current step goes past t_end in t_span, reduces h if it does
        if self.t + self.h > self.t_span[1]:
            h = self.t_span[1] - self.t
            self.solved = True
        else:
            h = self.h

        for i in range(self.n_stages):
            Q[i] = self.fun(self.t + self.C[i] * h, self.y + np.sum(Q.T * self.A[i]) * h)

        self.y += np.sum(Q.T * self.B) * h
        self.t += h
        self.iteration += 1


class RungeKutta4(RungeKutta):
    # Classic 4th order Runge Kutta
    C = np.array([0, 1 / 2, 1 / 2, 1])
    A = np.array([
        [0, 0, 0, 0],
        [1 / 2, 0, 0, 0],
        [0, 1 / 2, 0, 0],
        [0, 0, 1, 0],
    ])
    B = np.array([1 / 6, 1 / 3, 1 / 3, 1 / 6])
    n_stages = 4


class RungeKutta38Rule(RungeKutta):
    # Uses the 3/8 rule which has better accuracy than the classic RK4 but requires more computation
    C = np.array([0, 1 / 3, 2 / 3, 1])
    A = np.array([
        [0, 0, 0, 0],
        [1 / 3, 0, 0, 0],
        [-1 / 3, 1, 0, 0],
        [1, -1, 1, 0],
    ])
    B = np.array([1 / 8, 3 / 8, 3 / 8, 1 / 8])
    n_stages = 4
