class Euler:
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
        self.t_prev = self.t
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
