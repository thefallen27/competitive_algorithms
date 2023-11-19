from math import atan2
import sys
import matplotlib.pyplot as plt

class Point:
    def __init__(self, x, y):
        self.x = x
        self.y = y

def next_to_top(stack):
    p = stack.pop()
    res = stack[-1]
    stack.append(p)
    return res

def square_distance(p1, p2):
    return (p1.x - p2.x) ** 2 + (p1.y - p2.y) ** 2

def orientation(p, q, r):
    val = (q.y - p.y) * (r.x - q.x) - (q.x - p.x) * (r.y - q.y)
    if val == 0:
        return 0
    return 1 if val > 0 else 2

def convex_hull(points):
    n = len(points)
    if n < 3:
        return []

    # Finds the point with the lowest y-coordinate (and lowest x-coordinate if the same)
    y_minimum = points[0].y
    minimum_index = 0

    for i in range(1, n):
        y = points[i].y
        if y < y_minimum or (y == y_minimum and points[i].x < points[minimum_index].x):
            y_minimum = y
            minimum_index = i

    points[0], points[minimum_index] = points[minimum_index], points[0]

    # Sort the remaining points by polar angle in counterclockwise order
    p0 = points[0]
    points[1:] = sorted(points[1:], key=lambda p: (atan2(p.y - p0.y, p.x - p0.x), p.x, p.y))

    stack = [points[0], points[1]]

    for i in range(2, n):
        while len(stack) > 1 and orientation(next_to_top(stack), stack[-1], points[i]) != 2:
            stack.pop()
        stack.append(points[i])

    # Add the first point again to connect the convex hull for the drawing
    stack.append(points[0])

    return stack

def plot_convex_hull(points, convex_hull_points):
    x = [p.x for p in points]
    y = [p.y for p in points]

    convex_hull_x = [p.x for p in convex_hull_points]
    convex_hull_y = [p.y for p in convex_hull_points]

    plt.scatter(x, y, label="Points")
    plt.plot(convex_hull_x, convex_hull_y, color='red', label="Convex Hull")
    plt.xlabel("X")
    plt.ylabel("Y")
    plt.title("Convex Hull")
    plt.legend()
    plt.show()

if __name__ == "__main__":
    try:
        with open("convex_hull_input.txt", "r") as input_file:
            number_of_points = int(input_file.readline().strip())
            points = [Point(*map(int, input_file.readline().strip().split())) for _ in range(number_of_points)]

        convex_hull_points = convex_hull(points)

        print("Convex Hull points:")
        # Last element is removed as it is guaranteed to be the same
        # as the first one; it was added only for the drawing part of the
        # convex hull
        for p in convex_hull_points[:-1]: 
            print(p.x, p.y)

        plot_convex_hull(points, convex_hull_points)

    except FileNotFoundError:
        print("Failed to open input file.")
        sys.exit(1)