#include <fstream>
#include <iostream>
#include <limits>
#include <queue>
#include <vector>

struct edge
{
    int destination;
    int weight;
};

struct node
{
    std::vector<edge> neighbours;
};

std::vector<node>
ReadGraphFromFile(const std::string& filename, int& number_of_nodes)
{
    std::ifstream file(filename);
    if (!file)
    {
        std::cerr << "Error opening file " << filename << std::endl;
        exit(1);
    }

    file >> number_of_nodes;

    std::vector<node> graph(number_of_nodes);
    int source, destination, weight;

    while (file >> source >> destination >> weight)
    {
        graph[source].neighbours.push_back({ destination, weight });
        // Uncomment below line if you want the graph to be undirected
        // graph[destination].neighbours.push_back({ source, weight });
    }

    return graph;
}

std::vector<int>
Dijkstra(const std::vector<node>& graph, int start_node)
{
    std::vector<int> distances(graph.size(), std::numeric_limits<int>::max());
    distances[start_node] = 0;

    std::priority_queue<std::pair<int, int>, std::vector<std::pair<int, int>>, std::greater<>> pr_queue;
    pr_queue.push({ 0, start_node });

    while (!pr_queue.empty())
    {
        int distance = pr_queue.top().first;
        int current = pr_queue.top().second;
        pr_queue.pop();

        if (distance > distances[current])
        {
            continue;
        }

        for (const auto& graph_edge : graph[current].neighbours)
        {
            int neighbour = graph_edge.destination;
            int new_distance = distance + graph_edge.weight;

            if (new_distance < distances[neighbour])
            {
                distances[neighbour] = new_distance;
                pr_queue.push({ new_distance, neighbour });
            }
        }
    }

    return distances;
}

int main()
{
    std::ios::sync_with_stdio(false);
    std::cin.tie(NULL);
    std::string filename = "dijkstra_input.txt";

    int number_of_nodes;
    std::vector<node> graph = ReadGraphFromFile(filename, number_of_nodes);
    
    int start_node;
    std::cout << "Enter the starting node: ";
    std::cin >> start_node;

    std::vector<int> distances = Dijkstra(graph, start_node);

    std::cout << "Shortest paths from node " << start_node << ":" << std::endl;
    for (int i = 0; i < number_of_nodes; ++i)
    {
        if (distances[i] == std::numeric_limits<int>::max())
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
