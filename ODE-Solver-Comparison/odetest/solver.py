class Solver:
    def __init__(self, fun, t_span, y0, h):
        self.fun = fun  # Function to integrate
        self.t_span = t_span  # Span of t to integrate across, [t_start, t_end]
        self.y0 = y0  # Initial value, np.array([y1, y2, y3,...])
        self.y = y0.copy()  # Estimation at step
        self.iteration = 0  # Current iteration
        self.t = t_span[0]  # Current t
        self.t_prev = t_span[0]
        self.h = h  # Step size
        self.solved = False
        self.order = y0.size

    def step(self):
        NotImplemented

    def solve(self):
        while not self.solved:
            self.step()

        return self.t, self.y, self.iteration
