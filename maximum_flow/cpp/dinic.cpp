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
    FlowNetwork(int vertices) : graph_(vertices), levels_(vertices), next_edge_tracking_(vertices) {}

    void AddingEdge(int from, int to, int capacity)
    {
        graph_[from].push_back({ to, capacity, 0, graph_[to].size() });
        graph_[to].push_back({ from, 0, 0, graph_[from].size() - 1 });
    }

    bool BFS(int source, int sink)
    {
        fill(levels_.begin(), levels_.end(), -1);
        levels_[source] = 0;

        std::queue<int> q;
        q.push(source);

        while (!q.empty())
        {
            int current = q.front();
            q.pop();

            for (const Edge& edge : graph_[current])
            {
                if (levels_[edge.to] == -1 && edge.capacity > edge.flow)
                {
                    levels_[edge.to] = levels_[current] + 1;
                    q.push(edge.to);
                }
            }
        }

        return levels_[sink] != -1;
    }

    int DFS(int current, int sink, int minimum_capacity)
    {
        if (current == sink)
        {
            return minimum_capacity;
        }

        for (int& i = next_edge_tracking_[current]; i < graph_[current].size(); ++i)
        {
            Edge& edge = graph_[current][i];
            if (levels_[edge.to] == levels_[current] + 1 && edge.capacity > edge.flow)
            {
                int flow = DFS(edge.to, sink, std::min(minimum_capacity, edge.capacity - edge.flow));
                if (flow > 0)
                {
                    edge.flow += flow;
                    graph_[edge.to][edge.reverse_edge_index].flow -= flow;
                    return flow;
                }
            }
        }

        return 0;
    }

    int Dinic(int source, int sink)
    {
        int maximum_flow = 0;

        while (BFS(source, sink))
        {
            fill(next_edge_tracking_.begin(), next_edge_tracking_.end(), 0);
            while (int flow = DFS(source, sink, std::numeric_limits<int>::max()))
            {
                maximum_flow += flow;
            }
        }

        return maximum_flow;
    }

    std::vector<std::vector<Edge>> graph_;

private:
    std::vector<int> levels_;
    std::vector<int> next_edge_tracking_;
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

    int maximum_flow = network.Dinic(source, sink);
    std::cout << "Maximum Flow: " << maximum_flow << std::endl;

    return 0;
}
