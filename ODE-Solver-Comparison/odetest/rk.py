from .solver import Solver


class RungeKutta4(Solver):
    # Classic 4th order Runge Kutta
    def __init__(self, fun, t_span, y0, h):
        Solver.__init__(self, fun, t_span, y0, h)

    def step(self):
        self.t_prev = self.t

        # Checks to see if current step goes past t_end in t_span, reduces h if it does
        if self.t + self.h > self.t_span[1]:
            self.t = self.t_span[1]
            h = self.t - self.t_prev
        else:
            h = self.h

        k1 = self.fun(self.t, self.y)
        k2 = self.fun(self.t + 0.5 * h, self.y + k1 * 0.5 * h)
        k3 = self.fun(self.t + 0.5 * h, self.y + k2 * 0.5 * h)
        k4 = self.fun(self.t + h, self.y + k3 * h)

        self.y += (k1 + 2 * k2 + 2 * k3 + k4) * (h / 6)
        self.t += h
        self.iteration += 1


class RungeKutta38Rule(Solver):
    # Uses the 3/8 rule which has better accuracy than the classic RK4 but requires more computation
    def __init__(self, fun, t_span, y0, h):
        Solver.__init__(self, fun, t_span, y0, h)

    def step(self):
        self.t_prev = self.t

        # Checks to see if current step goes past t_end in t_span, reduces h if it does
        if self.t + self.h > self.t_span[1]:
            self.t = self.t_span[1]
            h = self.t - self.t_prev
        else:
            h = self.h

        k1 = self.fun(self.t, self.y)
        k2 = self.fun(self.t + (1 / 3) * h, self.y + k1 * (1 / 3) * h)
        k3 = self.fun(self.t + (2 / 3) * h, self.y + (k1 * (-1 / 3) + k2) * h)
        k4 = self.fun(self.t + h, self.y + (k1 - k2 + k3) * h)

        self.y += (k1 + 3 * k2 + 3 * k3 + k4) * (h / 8)
        self.t += h
        self.iteration += 1
