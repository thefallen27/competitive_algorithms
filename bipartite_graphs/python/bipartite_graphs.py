from collections import deque

def IsBipartite(graph, start):
    n = len(graph)
    colour = [-1] * n  # -1: uncoloured, 0: colour A, 1: colour B
    q = deque([start])
    colour[start] = 0

    while q:
        node = q.popleft()
        for neighbour in graph[node]:
            if colour[neighbour] == -1:
                colour[neighbour] = 1 - colour[node]  # Assigns the opposite colour
                q.append(neighbour)
            elif colour[neighbour] == colour[node]:
                return False

    return True


if __name__ == "__main__":
    try:
        with open("bipartite_graph_input_3.txt", "r") as input_file:
            nodes, edges = map(int, input_file.readline().split())

            graph = [[] for _ in range(nodes)]
            for _ in range(edges):
                from_node, to_node = map(int, input_file.readline().split())
                graph[from_node].append(to_node)
                graph[to_node].append(from_node)

    except IOError:
        print("Error: Unable to open input file")
        exit()

    is_bipartite = True
    colour = [-1] * nodes
    for i in range(nodes):
        if colour[i] == -1 and not IsBipartite(graph, i):
            is_bipartite = False
            break

    if is_bipartite:
        print("The graph is bipartite")
    else:
        print("The graph is not bipartite")
