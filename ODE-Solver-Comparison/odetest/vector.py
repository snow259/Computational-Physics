type Vector2 = Vector2


class Vector2:
    def __init__(self, x: int, y: int) -> None:
        self.x = x
        self.y = y

    def __eq__(self, other: Vector2) -> None:
        return self.x == other.x and self.y == other.y

    def __repr__(self) -> str:
        return f"[x: {self.x}, y: {self.y}]"

    def __str__(self) -> str:
        return f"[{self.x}, {self.y}]"

    # Adds and subtracts vector with another vector
    def __add__(self, vec2) -> Vector2:
        return Vector2(self.x + vec2.x, self.y + vec2.y)

    def __radd__(self, vec2):
        return Vector2(self.x + vec2.x, self.y + vec2.y)

    def __sub__(self, vec2):
        return Vector2(self.x - vec2.x, self.y - vec2.y)

    def __rsub__(self, vec2):
        return Vector2(self.x - vec2.x, self.y - vec2.y)

    # Multiplies vector with a scalar value
    def __mul__(self, scalar):
        return Vector2(self.x * scalar, self.y * scalar)

    def __rmul__(self, scalar):
        return Vector2(self.x * scalar, self.y * scalar)

    def mag(self):
        return (self.x**2 + self.y**2)**0.5
