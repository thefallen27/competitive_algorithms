#include <fstream>
#include <iostream>
#include <limits>
#include <queue>
#include <sstream>
#include <vector>

struct Edge
{
    int to;
    int capacity;
    int flow;
    size_t reverse_edge_index;
};

class FlowNetwork
{
public:
    FlowNetwork(int vertices) : vertices_(vertices), graph_(vertices) {}

    void AddingEdge(int from, int to, int capacity)
    {
        graph_[from].push_back({ to, capacity, 0, graph_[to].size() });
        graph_[to].push_back({ from, 0, 0, graph_[from].size() - 1 });
    }

    int EdmondsKarp(int source, int sink)
    {
        int maximum_flow = 0;

        while (int flow = AugmentingPath(source, sink))
        {
            maximum_flow += flow;
        }

        return maximum_flow;
    }

    std::vector<std::vector<Edge>> graph_;

private:
    int AugmentingPath(int source, int sink)
    {
        std::vector<int> parent(vertices_, -1);
        std::vector<int> minimum_capacity(vertices_, std::numeric_limits<int>::max());

        std::queue<int> q;
        q.push(source);
        parent[source] = source;
        while (!q.empty())
        {
            int current = q.front();
            q.pop();

            for (const Edge& edge : graph_[current])
            {
                if (parent[edge.to] == -1 && edge.capacity > edge.flow)
                {
                    parent[edge.to] = current;
                    minimum_capacity[edge.to] = std::min(minimum_capacity[current], edge.capacity - edge.flow);

                    if (edge.to == sink)
                    {
                        int path_capacity = minimum_capacity[sink];
                        int node = sink;
                        while (node != source)
                        {
                            int previous_node = parent[node];
                            graph_[previous_node][graph_[node][0].reverse_edge_index].flow -= path_capacity;
                            graph_[node][0].flow += path_capacity;
                            node = previous_node;
                        }

                        return path_capacity;
                    }

                    q.push(edge.to);
                }
            }
        }

        return 0;
    }

    int vertices_;
};

int main()
{
    std::ios::sync_with_stdio(false);
    std::cin.tie(NULL);
    std::ifstream input_file("maximum_flow_input.txt");

    if (!input_file)
    {
        std::cerr << "Failed to open the file." << std::endl;
        return 1;
    }
	
    int vertices, edges, source, sink;
    input_file >> vertices >> edges;
	
    std::cout << "Enter the source node: ";
    std::cin >> source;
    std::cout << "Enter the sink node: ";
    std::cin >> sink;

    FlowNetwork network(vertices);

    for (int i = 0; i < vertices; ++i)
    {
         network.graph_.push_back({});
    }

    for (int i = 0; i < edges; ++i)
    {
        int from, to, capacity;
        input_file >> from >> to >> capacity;
        network.AddingEdge(from, to, capacity);
    }

    input_file.close();

    int maximum_flow = network.EdmondsKarp(source, sink);
    std::cout << "Maximum Flow: " << maximum_flow << std::endl;

    return 0;
}
