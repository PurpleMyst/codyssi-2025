import enum
from math import prod
from pathlib import Path

SIDE = 80


class Faces(enum.IntEnum):
    UP = 0
    FRONT = 1
    DOWN = 2
    BACK = 3
    LEFT = 4
    RIGHT = 5


def clamp(n: int) -> int:
    while n > 100:
        n -= 100
    return n


def make_grid() -> list[list[int]]:
    return [[1 for _ in range(SIDE)] for _ in range(SIDE)]


def dominant_sum(grid: list[list[int]]) -> int:
    result = 0

    for i in range(SIDE):
        row_sum = 0
        col_sum = 0

        for j in range(SIDE):
            row_sum += grid[i][j]
            col_sum += grid[j][i]

        result = max(result, row_sum, col_sum)

    return result


def rotate_cw(grid: list[list[int]]) -> None:
    grid[:] = [list(reversed(col)) for col in zip(*grid)]


def rotate_ccw(grid: list[list[int]]) -> None:
    for _ in range(3):
        rotate_cw(grid)


class Cube:
    def __init__(self) -> None:
        self.vertical_loop = [Faces.UP, Faces.FRONT, Faces.DOWN, Faces.BACK]
        self.face_left = Faces.LEFT
        self.face_right = Faces.RIGHT

        self.grids_part2 = [make_grid() for _ in range(6)]
        self.grids_part3 = [make_grid() for _ in range(6)]
        self.absorption = [0 for _ in range(6)]

    def twist_U(self) -> None:
        self.vertical_loop = [*self.vertical_loop[1:], self.vertical_loop[0]]
        for grids in (self.grids_part2, self.grids_part3):
            rotate_ccw(grids[self.face_left])
            rotate_cw(grids[self.face_right])

    def twist_L(self) -> None:
        (
            self.face_left,
            self.face_right,
            self.vertical_loop[Faces.UP],
            self.vertical_loop[Faces.DOWN],
        ) = (
            self.vertical_loop[Faces.UP],
            self.vertical_loop[Faces.DOWN],
            self.face_right,
            self.face_left,
        )
        for grids in (self.grids_part2, self.grids_part3):
            rotate_ccw(grids[self.vertical_loop[Faces.FRONT]])
            rotate_cw(grids[self.face_right])
            rotate_cw(grids[self.face_right])
            rotate_cw(grids[self.vertical_loop[Faces.DOWN]])
            rotate_cw(grids[self.vertical_loop[Faces.DOWN]])
            rotate_cw(grids[self.vertical_loop[Faces.BACK]])

    def twist_D(self) -> None:
        for _ in range(3):
            self.twist_U()

    def twist_R(self) -> None:
        for _ in range(3):
            self.twist_L()

    def current_face(self) -> int:
        return self.vertical_loop[Faces.UP]

    def apply_instruction(self, instruction: str) -> None:
        target, _, value = instruction.partition(" - ")
        value = int(value.split()[1])

        self.absorption[self.current_face()] += value * (SIDE**2 if target == "FACE" else SIDE)

        grid = self.grids_part2[self.current_face()]
        match target.split()[0]:
            case "FACE":
                for y in range(SIDE):
                    for x in range(SIDE):
                        grid[y][x] = clamp(grid[y][x] + value)
            case "COL":
                x = int(target.split()[1]) - 1
                for y in range(SIDE):
                    grid[y][x] = clamp(grid[y][x] + value)
            case "ROW":
                y = int(target.split()[1]) - 1
                for x in range(SIDE):
                    grid[y][x] = clamp(grid[y][x] + value)
            case _:
                raise ValueError(f"Unknown instruction: {instruction}")

        match target.split()[0]:
            case "FACE":
                grid = self.grids_part3[self.current_face()]
                for y in range(SIDE):
                    for x in range(SIDE):
                        grid[y][x] = clamp(grid[y][x] + value)
            case "COL":
                x = int(target.split()[1]) - 1
                for _ in range(4):
                    grid = self.grids_part3[self.current_face()]
                    for y in range(SIDE):
                        grid[y][x] = clamp(grid[y][x] + value)
                    self.twist_U()
            case "ROW":
                y = int(target.split()[1]) - 1
                for _ in range(4):
                    grid = self.grids_part3[self.current_face()]
                    for x in range(SIDE):
                        grid[y][x] = clamp(grid[y][x] + value)
                    self.twist_R()
            case _:
                raise ValueError(f"Unknown instruction: {instruction}")

    def apply_twist(self, twist: str | None) -> None:
        match twist:
            case "U":
                self.twist_U()
            case "L":
                self.twist_L()
            case "D":
                self.twist_D()
            case "R":
                self.twist_R()
            case None:
                pass
            case _:
                raise ValueError(f"Unknown twist: {twist}")



def main() -> None:
    statement = (Path(__file__).parent / "src" / "input.txt").read_text()
    instructions, twists = statement.split("\n\n")
    cube = Cube()

    twists = iter(twists.strip())
    for instruction in instructions.splitlines():
        cube.apply_instruction(instruction)
        cube.apply_twist(next(twists, None))

    part1 = prod(sorted(cube.absorption)[-2:])
    print(part1)

    part2 = prod(map(dominant_sum, cube.grids_part2))
    print(part2)

    part3 = prod(map(dominant_sum, cube.grids_part3))
    print(part3)


if __name__ == "__main__":
    main()
