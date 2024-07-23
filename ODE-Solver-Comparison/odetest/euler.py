from .solver import Solver


class Euler(Solver):
    order_accuracy = 1
    order_error = 0

    def __init__(self, fun, t_span, y0, h):
        Solver.__init__(self, fun, t_span, y0, h)

    def step(self):
        self.t_prev = self.t

        # Checks to see if current step goes past t_end in t_span, reduces h if it does
        if self.t + self.h > self.t_span[1]:
            h = self.t_span[1] - self.t
            self.solved = True
        else:
            h = self.h

        fun_eval = self.fun(self.t, self.y)
        self.y += h * fun_eval

        self.t += h
        self.iteration += 1
