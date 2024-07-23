from .solver import Solver
import numpy as np


class RungeKutta(Solver):
    def step(self):
        # Q = np.zeros(self.n_stages)
        K = np.array([[0.] * self.order] * self.n_stages)
        self.t_prev = self.t

        # Checks to see if current step goes past t_end in t_span, reduces h if it does
        if self.t + self.h > self.t_span[1]:
            h = self.t_span[1] - self.t
            self.solved = True
        else:
            h = self.h

        K[0] = self.fun(self.t, self.y)
        # print(self.y)
        # print(Q)
        for i in range(1, self.n_stages):
            K[i] = self.fun(self.t + self.C[i] * h, self.y + (np.sum(K.T * self.A[i], axis=1).T * h))

        # print(self. y, K, self.B)
        self.y += np.dot(K.T, self.B) * h
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
    order_accuracy = 4
    order_error = 0


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
    order_accuracy = 4
    order_error = 0


class DormandPrince(RungeKutta):
    # Uses Dormand Prince coefficients. A 5th order method with a 4th order error check
    C = np.array([0, 1 / 5, 3 / 10, 4 / 5, 8 / 9, 1, 1])
    A = np.array([
        [0, 0, 0, 0, 0, 0, 0],
        [1 / 5, 0, 0, 0, 0, 0, 0],
        [3 / 40, 9 / 40, 0, 0, 0, 0, 0],
        [44 / 45, -56 / 15, 32 / 9, 0, 0, 0, 0],
        [19372 / 6561, -25360 / 2187, 64448 / 6561, -212 / 729, 0, 0, 0],
        [9017 / 3168, -355 / 33, 46732 / 5247, 49 / 176, -5103 / 18656, 0, 0],
        [35 / 384, 0, 500 / 1113, 125 / 192, -2187 / 6784, 11 / 84, 0],
    ])
    B = np.array([35 / 384, 0, 500 / 1113, 125 / 192, -2187 / 6784, 11 / 84, 0])
    E = B - np.array([5179 / 57600, 0, 7571 / 16695, 393 / 640, -92097 / 339200, 187 / 2100, 1 / 40])
    n_stages = 7
    order_accuracy = 5
    order_error = 4
