from collections import deque
import sys

INF = sys.maxsize

class HopcroftKarp:
    def __init__(self, set_U, set_V):
        self.U_ = set_U
        self.adjacency_ = [[] for _ in range(set_U)]
        self.pair_matching_from_U_ = [-1] * set_U
        self.pair_matching_from_V_ = [-1] * set_V
        self.distance_from_free_nodes_ = [0] * (set_U + 1)

    def add_edge(self, u, v):
        self.adjacency_[u].append(v)

    def maximum_matching(self):
        matching = 0
        while self.bfs():
            for u in range(self.U_):
                if self.pair_matching_from_U_[u] == -1 and self.dfs(u):
                    matching += 1
        return matching

    def bfs(self):
        q = deque()
        for u in range(self.U_):
            if self.pair_matching_from_U_[u] == -1:
                self.distance_from_free_nodes_[u] = 0
                q.append(u)
            else:
                self.distance_from_free_nodes_[u] = INF

        self.distance_from_free_nodes_[self.U_] = INF

        while q:
            u = q.popleft()
            if self.distance_from_free_nodes_[u] < self.distance_from_free_nodes_[self.U_]:
                for v in self.adjacency_[u]:
                    next_index = self.U_ if self.pair_matching_from_V_[v] == -1 else self.pair_matching_from_V_[v]
                    if self.distance_from_free_nodes_[next_index] == INF:
                        self.distance_from_free_nodes_[next_index] = self.distance_from_free_nodes_[u] + 1
                        q.append(next_index)

        return self.distance_from_free_nodes_[self.U_] != INF

    def dfs(self, u):
        if u == -1:
            return True
        for v in self.adjacency_[u]:
            next_index = self.pair_matching_from_V_[v]
            if next_index == -1 or (self.distance_from_free_nodes_[next_index] == self.distance_from_free_nodes_[u] + 1 and self.dfs(next_index)):
                self.pair_matching_from_V_[v] = u
                self.pair_matching_from_U_[u] = v
                return True

        self.distance_from_free_nodes_[u] = INF
        return False


if __name__ == "__main__":
    try:
        with open("hopcroft_karp_input.txt", "r") as input_file:
            nodes_set_U, nodes_set_V = map(int, input_file.readline().split())
            hk = HopcroftKarp(nodes_set_U, nodes_set_V)

            for line in input_file:
                from_u_set, to_v_set = map(int, line.split())
                hk.add_edge(from_u_set, to_v_set)

    except IOError:
        print("Failed to open input file.")
        exit()

    print("Maximum Cardinality Matching is", hk.maximum_matching())
