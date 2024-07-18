class Solver:
    def __init__(self, fun, t_span, y0, h):
        self.fun = fun  # Function to integrate
        self.t_span = t_span  # Span of t to integrate across, [t_start, t_end]
        self.y0 = y0  # Initial value, [n, ]
        self.y = y0  # Estimation at step
        self.iteration = 0  # Current iteration
        self.t = t_span[0]  # Current t
        self.t_prev = t_span[0]
        self.h = h  # Step size

    def step(self):
        NotImplemented

    def solve(self):
        while self.t <= self.t_span[1]:
            self.step()

        return self.t, self.y, self.iteration
