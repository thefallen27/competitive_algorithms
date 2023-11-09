from collections import deque

class Edge:
    def __init__(self, to, capacity, flow, reverse_edge_index):
        self.to = to
        self.capacity = capacity
        self.flow = flow
        self.reverse_edge_index = reverse_edge_index

class FlowNetwork:
    def __init__(self, vertices):
        self.graph_ = [[] for _ in range(vertices)]
        self.levels_ = [-1] * vertices
        self.next_edge_tracking_ = [0] * vertices

    def add_edge(self, from_, to, capacity):
        self.graph_[from_].append(Edge(to, capacity, 0, len(self.graph_[to])))
        self.graph_[to].append(Edge(from_, 0, 0, len(self.graph_[from_]) - 1))

    def bfs(self, source, sink):
        self.levels_ = [-1] * len(self.levels_)
        self.levels_[source] = 0

        q = deque([source])

        while q:
            current = q.popleft()
            for edge in self.graph_[current]:
                if self.levels_[edge.to] == -1 and edge.capacity > edge.flow:
                    self.levels_[edge.to] = self.levels_[current] + 1
                    q.append(edge.to)

        return self.levels_[sink] != -1

    def dfs(self, current, sink, minimum_capacity):
        if current == sink:
            return minimum_capacity

        while self.next_edge_tracking_[current] < len(self.graph_[current]):
            edge = self.graph_[current][self.next_edge_tracking_[current]]
            if self.levels_[edge.to] == self.levels_[current] + 1 and edge.capacity > edge.flow:
                flow = self.dfs(edge.to, sink, min(minimum_capacity, edge.capacity - edge.flow))
                if flow > 0:
                    edge.flow += flow
                    self.graph_[edge.to][edge.reverse_edge_index].flow -= flow
                    return flow

            self.next_edge_tracking_[current] += 1

        return 0

    def dinic(self, source, sink):
        maximum_flow = 0

        while self.bfs(source, sink):
            self.next_edge_tracking_ = [0] * len(self.next_edge_tracking_)
            while flow := self.dfs(source, sink, float('inf')):
                maximum_flow += flow

        return maximum_flow

if __name__ == '__main__':
    with open("maximum_flow_input.txt", 'r') as input_file:
        vertices, edges = map(int, input_file.readline().split())
        network = FlowNetwork(vertices)

        for _ in range(edges):
            from_, to, capacity = map(int, input_file.readline().split())
            network.add_edge(from_, to, capacity)

    source = int(input("Enter the source node: "))
    sink = int(input("Enter the sink node: "))

    if (source == sink):
        print("Source and sink are the same... No flow!")
        exit()
    else:
        maximum_flow = network.dinic(source, sink)
        print("Maximum Flow:", maximum_flow)
        