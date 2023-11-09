from collections import deque
import sys

class Edge:
    def __init__(self, to, capacity, flow):
        self.to = to
        self.capacity = capacity
        self.flow = flow

class FlowNetwork:
    def __init__(self, vertices):
        self.vertices_ = vertices
        self.graph_ = [[] for _ in range(vertices)]

    def add_edge(self, frm, to, capacity):
        self.graph_[frm].append(Edge(to, capacity, 0))
        self.graph_[to].append(Edge(frm, 0, 0))  # Reverse edge with initial flow 0

    def ford_fulkerson(self, source, sink):
        max_flow = 0
        while flow := self.augmenting_path(source, sink):
            max_flow += flow
        return max_flow

    def augmenting_path(self, source, sink):
        parent = [-1] * self.vertices_
        minimum_capacity = [sys.maxsize] * self.vertices_
        
        q = deque()
        q.append(source)

        while q:
            current = q.popleft()
            for edge in self.graph_[current]:
                if parent[edge.to] == -1 and edge.capacity > edge.flow:
                    parent[edge.to] = current
                    minimum_capacity[edge.to] = min(minimum_capacity[current], edge.capacity - edge.flow)
                    if edge.to == sink:
                        path_capacity = minimum_capacity[sink]
                        node = sink
                        while node != source:
                            previous_node = parent[node]
                            for back_edge in self.graph_[previous_node]:
                                if back_edge.to == node:
                                    back_edge.flow += path_capacity
                                    break
                            for forward_edge in self.graph_[node]:
                                if forward_edge.to == previous_node:
                                    forward_edge.flow -= path_capacity
                                    break
                            node = previous_node
                        return path_capacity
                    q.append(edge.to)
        return 0

if __name__ == "__main__":
    with open("maximum_flow_input.txt", "r") as input_file:
        vertices, edges = map(int, input_file.readline().split())

        network = FlowNetwork(vertices)

        for _ in range(edges):
            frm, to, capacity = map(int, input_file.readline().split())
            network.add_edge(frm, to, capacity)

    source = int(input("Enter the source node: "))
    sink = int(input("Enter the sink node: "))

    if (source == sink):
        print("Source and sink are the same... No flow!")
        exit()
    else:
        maximum_flow = network.ford_fulkerson(source, sink)
        print("Maximum Flow:", maximum_flow)
