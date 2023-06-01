#include <iostream>
#include <fstream>
#include <string>
#include <vector>
#include <queue>
#include <limits>

struct edge 
{
    int destination;
    int weight;
};

class graph 
{
public:
    explicit graph(int number_of_nodes) : adjacency_list_(number_of_nodes) {}

    void add_edge(int source, int destination, int weight) 
    {
        adjacency_list_[source].push_back({ destination, weight });
    }

    std::pair<std::vector<int>, int> shortest_path(int start, int end) const
    {
        std::vector<int> distances(adjacency_list_.size(), std::numeric_limits<int>::max());
        std::vector<int> previous(adjacency_list_.size(), -1);
        std::vector<bool> visited(adjacency_list_.size(), false);

        distances[start] = 0;
		
        std::queue<int> queue;
        queue.push(start);
        visited[start] = true;

        while (!queue.empty()) 
        {
            int current = queue.front();
            queue.pop();

            for (const edge& bfs_edge : adjacency_list_[current]) 
            {
                int neighbour = bfs_edge.destination;
                int weight = bfs_edge.weight;

                if (!visited[neighbour]) 
                {
                    visited[neighbour] = true;
                    queue.push(neighbour);
                }

                if (distances[current] + weight < distances[neighbour]) 
                {
                    distances[neighbour] = distances[current] + weight;
                    previous[neighbour] = current;
                }
            }
        }

        std::vector<int> path;
        int current = end;
        while (current != -1) 
        {
            path.push_back(current);
            current = previous[current];
        }

        std::reverse(path.begin(), path.end());
        return { path, distances[end] };
    }

private:
    std::vector<std::vector<edge>> adjacency_list_;
};

int main() 
{
    std::ios::sync_with_stdio(false);
    std::cin.tie(NULL);

    std::string filename = "bfs_input.txt";
    std::ifstream file(filename);
    
    if (!file) 
    {
        std::cerr << "Failed to open the file." << std::endl;
        return 1;
    }

    int number_of_nodes, number_of_edges;
    file >> number_of_nodes >> number_of_edges;

    graph bfs_graph(number_of_nodes);

    for (int i = 0; i < number_of_edges; ++i) 
    {
        int source, destination, weight;
        file >> source >> destination >> weight;
        bfs_graph.add_edge(source, destination, weight);
    }

    int start, end;
    std::cout << "Enter starting node: ";
    std::cin >> start;
    std::cout << "Enter end node: ";
    std::cin >> end;

    std::pair<std::vector<int>, int> result = bfs_graph.shortest_path(start, end);
    std::vector<int> path = result.first;
    int shortest_distance = result.second;

    if (path[0] != start) 
    {
        std::cout << "No path exists between the specified nodes." << std::endl;
        return 0;
    }

    std::cout << "Shortest path: ";
    for (int node : path) 
    {
        std::cout << node << " ";
    }

    std::cout << std::endl << "Shortest distance: " << shortest_distance << std::endl;

    return 0;
}


