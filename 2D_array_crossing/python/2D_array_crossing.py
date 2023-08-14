import heapq
from typing import List, Tuple

MAX = float('inf')
OBSTACLE = 0

class Cell:
    def __init__(self, r, c, cost):
        self.row = r
        self.col = c
        self.cost = cost

    def __lt__(self, other):
        return self.cost < other.cost

def FindShortestPath(grid: List[List[int]], start: Tuple[int, int], end: Tuple[int, int]) -> int:
    rows = len(grid)
    columns = len(grid[0])

    distance = [[MAX for _ in range(columns)] for _ in range(rows)]
    parent = [[(-1, -1) for _ in range(columns)] for _ in range(rows)]

    distance[start[0]][start[1]] = grid[start[0]][start[1]]
    pq = []
    heapq.heappush(pq, Cell(start[0], start[1], distance[start[0]][start[1]]))

    while pq:
        current = heapq.heappop(pq)

        if (current.row, current.col) == end:
            break

        next_row = [0, 1]
        next_column = [1, 0]

        for i in range(2):
            new_row = current.row + next_row[i]
            new_column = current.col + next_column[i]

            if 0 <= new_row < rows and 0 <= new_column < columns and grid[new_row][new_column] != OBSTACLE:
                new_cost = current.cost + grid[new_row][new_column]
                if new_cost < distance[new_row][new_column]:
                    distance[new_row][new_column] = new_cost
                    parent[new_row][new_column] = (current.row, current.col)
                    heapq.heappush(pq, Cell(new_row, new_column, new_cost))

    shortest_path_sum = distance[end[0]][end[1]]
    if shortest_path_sum == MAX:
        return -1

    path = []
    current = end
    path.append(current)

    while current != start:
        current = parent[current[0]][current[1]]
        path.append(current)

    path.reverse()

    print("Shortest path with smallest sum:", end=" ")
    for cell in path:
        print(f"({cell[0]}, {cell[1]})", end=" -> ")
    print()

    return shortest_path_sum

if __name__ == '__main__':
    filename = "2D_array_crossing_input.txt"

    try:
        with open(filename) as input_file:
            rows, columns = map(int, input_file.readline().strip().split())
            grid = [list(map(int, input_file.readline().strip().split())) for _ in range(rows)]

    except FileNotFoundError:
        print(f"Failed to open file: {filename}")
        exit()

    start = tuple(map(int, input("Enter the starting point (row col): ").split()))
    end = tuple(map(int, input("Enter the ending point (row col): ").split()))

    shortest_path_sum = FindShortestPath(grid, start, end)
    if shortest_path_sum == -1:
        print("No valid path found.")
    else:
        print("Cost of crossing: ", shortest_path_sum)