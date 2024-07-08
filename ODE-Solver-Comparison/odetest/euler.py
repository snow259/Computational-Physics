from .solver import Solver


class Euler(Solver):
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

        self.t += h
        self.iteration += 1
        fun_eval = self.fun(self.t, self.y)

        for i in range(len(self.y0)):
            self.y[i] += h * fun_eval[i]

    def solve(self):
        while self.t <= self.t_span[1]:
            self.step()
            # self.y, self.t = self.step(self.fun, self.t, self.y, self.h)

        return self.t, self.y, self.iteration
