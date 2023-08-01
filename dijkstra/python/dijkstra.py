import heapq

class Edge:
    def __init__(self, destination, weight):
        self.destination = destination
        self.weight = weight

class Node:
    def __init__(self):
        self.neighbours = []

def ReadGraphFromFile(filename):
    with open(filename, 'r') as input_file:
        number_of_nodes = int(input_file.readline())
        graph = [Node() for _ in range(number_of_nodes)]
        
        for line in input_file:
            source, destination, weight = map(int, line.strip().split())
            graph[source].neighbours.append(Edge(destination, weight))
            # Uncomment below line if you want the graph to be undirected
            # graph[destination].neighbours.append(Edge(source, weight))

    return graph

def Dijkstra(graph, start):
    distances = [float('inf')] * len(graph)
    distances[start] = 0

    priority_queue = [(0, start)]

    while priority_queue:
        distance, current = heapq.heappop(priority_queue)

        if distance > distances[current]:
            continue

        for edge in graph[current].neighbours:
            neighbour = edge.destination
            new_distance = distance + edge.weight

            if new_distance < distances[neighbour]:
                distances[neighbour] = new_distance
                heapq.heappush(priority_queue, (new_distance, neighbour))

    return distances

if __name__ == "__main__":
    filename = "dijkstra_input.txt"
    graph = ReadGraphFromFile(filename)

    start = int(input("Enter the starting node: "))
    distances = Dijkstra(graph, start)

    print(f"Shortest paths from node {start}:")
    for i, distance in enumerate(distances):
        if i != start:
            if distance == float('inf'):
                print(f"No path exists from {start} to {i}")
            else:
                print(f"Shortest path from {start} to {i}: {distance}")
