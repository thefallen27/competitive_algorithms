from collections import deque

class Edge:
    def __init__(self, destination, weight):
        self.destination = destination
        self.weight = weight

class Graph:
    def __init__(self, number_of_vertices):
        self.adjacency_list = [[] for _ in range(number_of_vertices)]

    def AddEdge(self, source, destination, weight):
        self.adjacency_list[source].append(Edge(destination, weight))

    def ShortestPath(self, start, end):
        number_of_vertices = len(self.adjacency_list)
        distances = [float('inf')] * number_of_vertices
        previous = [-1] * number_of_vertices
        visited = [False] * number_of_vertices

        distances[start] = 0

        queue = deque()
        queue.append(start)
        visited[start] = True

        while queue:
            current = queue.popleft()

            for edge in self.adjacency_list[current]:
                neighbour = edge.destination
                weight = edge.weight

                if not visited[neighbour]:
                    visited[neighbour] = True
                    queue.append(neighbour)

                if distances[current] + weight < distances[neighbour]:
                    distances[neighbour] = distances[current] + weight
                    previous[neighbour] = current

        path = []
        current = end
        while current != -1:
            path.append(current)
            current = previous[current]

        path.reverse()
        return path, distances[end]

if __name__ == "__main__":
    filename = "bfs_input.txt"
    with open(filename, 'r') as file:
        number_of_vertices, number_of_edges = map(int, file.readline().split())

        graph = Graph(number_of_vertices)

        for _ in range(number_of_edges):
            source, destination, weight = map(int, file.readline().split())
            graph.AddEdge(source, destination, weight)

    start = int(input("Enter the starting node: "))
    end = int(input("Enter the end node: "))

    path, shortest_distance = graph.ShortestPath(start, end)

    if path[0] != start:
        print("No path exists between the specified nodes.")
    else:
        print("Shortest path:", " ".join(str(node) for node in path))
        print("Shortest distance:", shortest_distance)
