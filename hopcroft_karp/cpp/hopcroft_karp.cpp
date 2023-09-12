#include <cstring>
#include <fstream>
#include <iostream>
#include <limits>
#include <queue>
#include <vector>

const int INF = std::numeric_limits<int>::max();

class HopcroftKarp
{
public:
    HopcroftKarp(int set_U, int set_V) : U_(set_U), V_(set_V), adjacency_(set_U),
             pair_matching_from_U_(set_U, -1), pair_matching_from_V_(set_V, -1),
             distance_from_free_nodes_(U_ + 1) {}

    void AddEdge(int u, int v)
    {
        adjacency_[u].push_back(v);
    }

    int MaximumMatching()
    {
        int matching = 0;
        while (BFS())
        {
            for (int u = 0; u < U_; ++u)
            {
                if (pair_matching_from_U_[u] == -1 && DFS(u))
                {
                    matching++;
                }
            }
        }

        return matching;
    }

private:
    bool BFS()
    {
        std::queue<int> q;
        for (int u = 0; u < U_; ++u)
        {
            if (pair_matching_from_U_[u] == -1)
            {
                distance_from_free_nodes_[u] = 0;
                q.push(u);
            }
            else
            {
                distance_from_free_nodes_[u] = INF;
            }
        }

        distance_from_free_nodes_[U_] = INF;

        while (!q.empty())
        {
            int u = q.front();
            q.pop();

            if (distance_from_free_nodes_[u] < distance_from_free_nodes_[U_])
            {
                for (int v : adjacency_[u])
                {
                    if (pair_matching_from_V_[v] == -1 || (pair_matching_from_V_[v] != -1
                        && distance_from_free_nodes_[pair_matching_from_V_[v]] == INF))
                    {
                        int nextIndex = (pair_matching_from_V_[v] == -1 ? U_ : pair_matching_from_V_[v]);
                        distance_from_free_nodes_[nextIndex] = distance_from_free_nodes_[u] + 1;
                        q.push(nextIndex);
                    }
                }
            }
        }

        return distance_from_free_nodes_[U_] != INF;
    }

    bool DFS(int u)
    {
        if (u == -1) return true;
        for (int v : adjacency_[u])
        {
            if (pair_matching_from_V_[v] == -1
                || (distance_from_free_nodes_[pair_matching_from_V_[v]] == distance_from_free_nodes_[u] + 1
                && DFS(pair_matching_from_V_[v])))
            {
                pair_matching_from_V_[v] = u;
                pair_matching_from_U_[u] = v;
                return true;
            }
        }

        distance_from_free_nodes_[u] = INF;
        return false;
    }

    int U_, V_;
    std::vector<std::vector<int>> adjacency_;
    std::vector<int> pair_matching_from_U_, pair_matching_from_V_, distance_from_free_nodes_;
};

int main()
{
    std::ios::sync_with_stdio(false);
    std::cin.tie(NULL);
    std::ifstream input_file("hopcroft_karp_input.txt");

    if (!input_file)
    {
        std::cout << "Failed to open input file." << std::endl;
        return 1;
    }

    int nodes_set_U, nodes_set_V, from_u_set, to_v_set;
    input_file >> nodes_set_U >> nodes_set_V;
    HopcroftKarp hk(nodes_set_U, nodes_set_V);

    while (input_file >> from_u_set >> to_v_set)
    {
        hk.AddEdge(from_u_set, to_v_set);
    }

    std::cout << "Maximum Cardinality Matching is " << hk.MaximumMatching() << std::endl;

    return 0;
}
