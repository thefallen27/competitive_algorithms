#include <iostream>
#include <fstream>
#include <vector>
#include <stack>
#include <unordered_set>

void
DFS(const std::vector<std::vector<int>>& graph, int start_node, std::vector<bool>& visited)
{
    std::stack<int> st;
    st.push(start_node);

    while (!st.empty())
    {
        int curr_node = st.top();
        st.pop();

        if (!visited[curr_node])
        {
            visited[curr_node] = true;
            std::cout << curr_node << " ";

            for (int neighbour : graph[curr_node])
            {
                if (!visited[neighbour])
                {
                    st.push(neighbour);
                }
            }
        }
    }
}

std::vector<std::vector<int>>
ReadGraphFromFile(const std::string& filename)
{
    std::ifstream file(filename);
    if (!file.is_open())
    {
        std::cerr << "Error: Unable to open file " << filename << "\n";
        exit(1);
    }

    int num_of_nodes, num_of_edges;
    file >> num_of_nodes >> num_of_edges;

    std::vector<std::vector<int>> graph(num_of_nodes);
    int u, v;
    for (int i = 0; i < num_of_edges; ++i)
    {
        file >> u >> v;
        graph[u].push_back(v);
        graph[v].push_back(u);
    }

    file.close();
    return graph;
}

int main()
{
    std::ios::sync_with_stdio(false);
    std::cin.tie(NULL);
    std::string filename = "dfs_input.txt";

    std::vector<std::vector<int>> graph = ReadGraphFromFile(filename);

    size_t num_of_nodes = graph.size();
    std::vector<bool> visited(num_of_nodes, false);

    int start_node;
    std::cout << "Enter starting node: ";
    std::cin >> start_node;

    std::cout << "DFS traversal: ";
    DFS(graph, start_node, visited);
    std::cout << "\n";

    return 0;
}
