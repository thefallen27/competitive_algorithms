import math

Coordinates = tuple

def dfs(adjacency_list: list[list[int]], u_set: list[int], v_set: list[int], 
        visited_nodes: list[int], current: int) -> bool:
    if visited_nodes[current]:
        return False

    visited_nodes[current] = True

    for neighbour in adjacency_list[current]:
        if v_set[neighbour] == -1 or dfs(adjacency_list, u_set, v_set, visited_nodes, v_set[neighbour]):
            u_set[current] = neighbour
            v_set[neighbour] = current
            return True

    return False

def maximum_matching(adjacency_list: list[list[int]], u_nodes: int, v_nodes: int) -> int:
    u_set = [-1] * u_nodes
    v_set = [-1] * v_nodes
    visited = [False] * u_nodes
    match_found = True

    while match_found:
        match_found = False
        visited = [False] * u_nodes  # Reset visited
        for i in range(u_nodes):
            if u_set[i] == -1:
                if dfs(adjacency_list, u_set, v_set, visited, i):
                    match_found = True

    return sum(1 for value in u_set if value != -1)

def bfs(midway: float, distance: list[list[float]], n: int, r: int, b: int) -> bool:
    adjacency_list = [[] for _ in range(r)]
    for i in range(r):
        for j in range(b):
            if distance[i][j] < midway:
                adjacency_list[i].append(j)

    return r + b - maximum_matching(adjacency_list, r, b) >= n

def euclidean_distance(p1: Coordinates, p2: Coordinates) -> float:
    d1 = (p1[0] - p2[0]) ** 2
    d2 = (p1[1] - p2[1]) ** 2
    return math.sqrt(d1 + d2)

if __name__ == "__main__":
    n, r, b = map(int, input().split())

    redberry = []
    for _ in range(r):
        x, y = map(float, input().split())
        redberry.append((x, y))

    blueberry = []
    for _ in range(b):
        x, y = map(float, input().split())
        blueberry.append((x, y))

    distances = []
    for i in range(r):
        row = []
        for j in range(b):
            row.append(euclidean_distance(redberry[i], blueberry[j]))
        distances.append(row)

    epsilon = 1e-9
    low, high = 0, 1e19
    while high - low > epsilon:
        midway = (high + low) / 2
        if bfs(midway, distances, n, r, b):
            low = midway
        else:
            high = midway

    print(f"{low:.8f}")
