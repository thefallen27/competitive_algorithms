#include <fstream>
#include <iostream>
#include <limits>
#include <queue>
#include <vector>

constexpr int INF = std::numeric_limits<int>::max();

struct edge 
{
    int destination;
    int weight;
};

void 
ReadGraphFromFile(const std::string& filename, std::vector<std::vector<edge>>& graph) 
{
    std::ifstream file(filename);
    if (!file) 
    {
        std::cerr << "Error opening file " << filename << std::endl;
        exit(1);
    }

    int number_of_nodes, number_of_edges;
    file >> number_of_nodes >> number_of_edges;

    graph.resize(number_of_nodes);

    for (int i = 0; i < number_of_edges; ++i) 
    {
        int source, destination, weight;
        file >> source >> destination >> weight;

        graph[source].push_back({ destination, weight });
        // Uncomment below line if you want the graph to be undirected
        // graph[destination].push_back({ source, weight });
    }

    file.close();
}

void 
Dijkstra(const std::vector<std::vector<edge>>& graph, int start_node, std::vector<int>& distances) 
{
    long long int number_of_nodes = graph.size();
    distances.assign(number_of_nodes, INF);
    distances[start_node] = 0;

    std::priority_queue<std::pair<int, int>, std::vector<std::pair<int, int>>, std::greater<>> pr_queue;
    pr_queue.push({ 0, start_node });

    while (!pr_queue.empty()) 
    {
        int dist = pr_queue.top().first;
        int u = pr_queue.top().second;
        pr_queue.pop();

        if (dist > distances[u]) 
        {
            continue;
        }

        for (const auto& edge : graph[u]) 
        {
            int v = edge.destination;
            int weight = edge.weight;

            if (dist + weight < distances[v]) 
            {
                distances[v] = dist + weight;
                pr_queue.push({ distances[v], v });
            }
        }
    }
}

int main() 
{
    std::ios::sync_with_stdio(false);
    std::cin.tie(NULL);
    std::string filename = "dijkstra_input.txt";

    std::vector<std::vector<edge>> graph;
    ReadGraphFromFile(filename, graph);

    int start_node;
    std::cout << "Enter the starting node: ";
    std::cin >> start_node;

    std::vector<int> distances;
    Dijkstra(graph, start_node, distances);

    std::cout << "Shortest paths from node " << start_node << ":" << std::endl;
    for (int i = 0; i < distances.size(); ++i) 
    {
        if (distances[i] == INF) 
        {
            std::cout << "No path from node " << start_node << " to node " << i << std::endl;
        }
        else 
        {
            std::cout << "Node " << i << ": Distance = " << distances[i] << std::endl;
        }
    }

    return 0;
}
