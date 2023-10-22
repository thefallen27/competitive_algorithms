#include <fstream>
#include <iostream>
#include <queue>
#include <sstream>
#include <vector>

bool 
IsBipartite(const std::vector<std::vector<int>>& graph, int start)
{
    size_t n = graph.size();
    std::vector<int> colour(n, -1); // -1: uncoloured, 0: colour A, 1: colour B
    std::queue<int> q;
    q.push(start);
    colour[start] = 0;

    while (!q.empty())
    {
        int node = q.front();
        q.pop();

        for (int neighbour : graph[node])
        {
            if (colour[neighbour] == -1)
            {
                colour[neighbour] = 1 - colour[node]; // Assigns the opposite colour
                q.push(neighbour);
            }
            else if (colour[neighbour] == colour[node])
            {
                return false;
            }
        }
    }

    return true;
}

int main()
{
    std::ios::sync_with_stdio(false);
    std::cin.tie(NULL);

    std::ifstream inputFile("bipartite_graph_input_2.txt");
    if (!inputFile)
    {
        std::cout << "Error: Unable to open input file" << std::endl;
        return 1;
    }

    int nodes, edges;
    inputFile >> nodes >> edges;

    std::vector<std::vector<int>> graph(nodes);
    for (int i = 0; i < edges; ++i)
    {
        int from_node, to_node;
        inputFile >> from_node >> to_node;
        graph[from_node].push_back(to_node);
        graph[to_node].push_back(from_node);
    }

    inputFile.close();

    bool is_bipartite = true;
    std::vector<int> colour(nodes, -1);
    for (int i = 0; i < nodes; ++i)
    {
        if (colour[i] == -1 && !IsBipartite(graph, i))
        {
            is_bipartite = false;
            break;
        }
    }

    if (is_bipartite)
    {
        std::cout << "The graph is bipartite" << std::endl;
    }
    else
    {
        std::cout << "The graph is not bipartite" << std::endl;
    }

    return 0;
}
