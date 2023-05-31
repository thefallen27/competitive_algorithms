def DFS(start_node, graph, visited):

    stack = []
    stack.append(start_node)

    while(stack):
        curr_node = stack[-1]
        stack.pop()

        if not visited[curr_node]:
            visited[curr_node] = True
            print(curr_node, end = ' ')

            for neighbour in graph[curr_node]:
                if not visited[neighbour]:
                    stack.append(neighbour)

def GraphCreation(filename):

    with open(filename, 'r') as f:  
        numofnodes,numofedges = map(int,f.readline().split())
        graph = [[] for i in range(numofnodes)]

        for i in range(numofedges):
            pnode,cnode = map(int,f.readline().split())
            graph[pnode].append(cnode)
            graph[cnode].append(pnode)
       
    return graph


if __name__ == '__main__':
    graph = GraphCreation("dfs_input.txt")
    visited = [False]*len(graph)
    start_node = int(input("Enter starting node: "))

    print("DFS travesal: ", end = '')
    DFS(start_node, graph, visited)
    print()
