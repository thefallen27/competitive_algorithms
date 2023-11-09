
from collections import deque
import sys

class Edge:
    def __init__(self, to, capacity, flow, reverse_edge_index):
        self.to = to
        self.capacity = capacity
        self.flow = flow
        self.reverse_edge_index = reverse_edge_index

class FlowNetwork:
    def __init__(self, vertices):
        self.vertices = vertices
        self.graph = [[] for _ in range(vertices)]

    def add_edge(self, frm, to, capacity):
        self.graph[frm].append(Edge(to, capacity, 0, len(self.graph[to])))
        self.graph[to].append(Edge(frm, 0, 0, len(self.graph[frm]) - 1))

    def edmonds_karp(self, source, sink):
        maximum_flow = 0
        while flow := self.augmenting_path(source, sink):
            maximum_flow += flow
        return maximum_flow

    def augmenting_path(self, source, sink):
        parent = [-1] * self.vertices
        minimum_capacity = [sys.maxsize] * self.vertices

        q = deque([source])
        parent[source] = source

        while q:
            current = q.popleft()
            for edge in self.graph[current]:
                if parent[edge.to] == -1 and edge.capacity > edge.flow:
                    parent[edge.to] = current
                    minimum_capacity[edge.to] = min(minimum_capacity[current], edge.capacity - edge.flow)
                    
                    if edge.to == sink:
                        path_capacity = minimum_capacity[sink]
                        node = sink
                        while node != source:
                            prev_node = parent[node]
                            edge_index = None
                            reverse_edge_index = None
                            for ei, edge in enumerate(self.graph[prev_node]):
                                if edge.to == node:
                                    edge_index = ei
                                    reverse_edge_index = edge.reverse_edge_index
                                    break
                            self.graph[prev_node][edge_index].flow += path_capacity
                            self.graph[node][reverse_edge_index].flow -= path_capacity
                            node = prev_node

                        return path_capacity
                    q.append(edge.to)
        return 0

if __name__ == "__main__":
    with open("maximum_flow_input.txt", 'r') as f:
        vertices, edges = map(int, f.readline().split())

        network = FlowNetwork(vertices)
      
        for _ in range(edges):
            frm, to, capacity = map(int, f.readline().split())
            network.add_edge(frm, to, capacity)

        source = int(input("Enter the source node: "))
        sink = int(input("Enter the sink node: "))

        if (source == sink):
            print("Source and sink are the same... No flow!")
            exit()
        else:
            maximum_flow = network.edmonds_karp(source, sink)
            print(f"Maximum Flow: {maximum_flow}")
