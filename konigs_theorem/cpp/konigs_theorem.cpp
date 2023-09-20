#include <algorithm>
#include <cmath>
#include <functional>
#include <iomanip>
#include <iostream>
#include <ranges>
#include <vector>

using coordinates = std::pair<long double, long double>;

bool
DFS(const std::vector<std::vector<int>>& adjacency_list, std::vector<int>& U_set, 
      std::vector<int>& V_set, std::vector<int>& visited_nodes, int current)
{
    if (visited_nodes[current])
    {
        return false;
    }

    visited_nodes[current] = true;

    for (const auto& neighbour : adjacency_list[current])
    {
        if (V_set[neighbour] == -1 
           || DFS(adjacency_list, U_set, V_set, visited_nodes, V_set[neighbour]))
        {
            U_set[current] = neighbour;
            V_set[neighbour] = current;
            return true;
        }
    }

    return false;
}

std::ptrdiff_t
MaximumMatching(std::vector<std::vector<int>>& adjacency_list, int u_nodes, int v_nodes)
{
    std::vector<int> U_set(u_nodes, -1), V_set(v_nodes, -1), visited(u_nodes, 0);
    bool match_found = true;

    while (match_found)
    {
        match_found = false;
        for (int i = 0; i < u_nodes; ++i)
        {
            if (U_set[i] == -1)
            {
                match_found |= DFS(adjacency_list, U_set, V_set, visited, i);
            }
        }
    }

    return std::ranges::count_if(U_set, std::not_fn([](int value) { return value == -1; }));
}

bool
BFS(long double midway, const std::vector<std::vector<long double>>& distance, int n, int r, int b)
{
    std::vector<std::vector<int>> adjacency_list(r);
    for (int i = 0; i < r; ++i)
    {
        for (int j = 0; j < b; ++j)
        {
            if (distance[i][j] < midway)
            {
                adjacency_list[i].push_back(j);
            }
        }
    }

    return static_cast<long long>(r) + b - MaximumMatching(adjacency_list, r, b) >= n;
}

long double
EuclideanDistance(const coordinates& p1, const coordinates& p2)
{
    long double d1 = std::pow(p1.first - p2.first, 2);
    long double d2 = std::pow(p1.second - p2.second, 2);
    return std::sqrt(d1 + d2);
}

int main()
{
    std::ios::sync_with_stdio(false);
    std::cin.tie(NULL);

    int n, r, b;
    std::cin >> n >> r >> b;

    std::vector<coordinates> redberry(r);
    std::vector<coordinates> blueberry(b);

    for (auto& [x, y] : redberry)
    {
        std::cin >> x >> y;
    }
    
    for (auto& [x, y] : blueberry)
    {
        std::cin >> x >> y;
    }

    std::vector distances(r, std::vector<long double>(b));
    for (int i = 0; i < r; ++i)
    {
        for (int j = 0; j < b; ++j)
        {
            distances[i][j] = EuclideanDistance(redberry[i], blueberry[j]);
        }
    }

    constexpr long double epsilon = 1e-9;
    long double low = 0, high = 1e19;
    while (high - low > epsilon)
    {
        long double midway = (high + low) / 2;
        if (BFS(midway, distances, n, r, b))
        {
            low = midway;
        }
        else
        {
            high = midway;
        }
    }

    std::cout << std::fixed << std::setprecision(8) << low << '\n';
}
