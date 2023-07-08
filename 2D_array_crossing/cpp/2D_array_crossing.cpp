#include <fstream>
#include <iostream>
#include <limits>
#include <queue>
#include <vector>

constexpr int MAX = std::numeric_limits<int>::max();
constexpr int OBSTACLE = 0;

struct Cell 
{
    int row;
    int column;
    int cost;

    Cell(int r, int c, int cost) : row(r), column(c), cost(cost) {}
};

struct CompareCell 
{
    bool operator()(const Cell& c1, const Cell& c2) { return c1.cost > c2.cost; }
};

int 
FindShortestPath(const std::vector<std::vector<int>>& grid, 
                 const std::pair<int, int>& start,
                 const std::pair<int, int>& end) 
{
    size_t rows = grid.size();
    size_t columns = grid[0].size();

    std::vector<std::vector<int>> distance(rows, std::vector<int>(columns, MAX));
    std::vector<std::vector<std::pair<int, int>>> parent(rows, std::vector<std::pair<int, int>>(columns));

    distance[start.first][start.second] = grid[start.first][start.second];
    std::priority_queue<Cell, std::vector<Cell>, CompareCell> pq;
    pq.emplace(start.first, start.second, distance[start.first][start.second]);

    while (!pq.empty())
    {
        Cell current = pq.top();
        pq.pop();

        if (current.row == end.first && current.column == end.second)
        {
            break;
        }

        static const int next_row[] = { 0, 1 };
        static const int next_column[] = { 1, 0 };

        for (int i = 0; i < 2; ++i)
        {
            int new_row = current.row + next_row[i];
            int new_column = current.column + next_column[i];

            if (new_row >= 0 && new_row < rows && new_column >= 0 && 
                new_column < columns && grid[new_row][new_column] != OBSTACLE)
            {
                int new_cost = current.cost + grid[new_row][new_column];
                if (new_cost < distance[new_row][new_column])
                {
                    distance[new_row][new_column] = new_cost;
                    parent[new_row][new_column] = { current.row, current.column };
                    pq.emplace(new_row, new_column, new_cost);
                }
            }
        }
    }

    int shortest_path_sum = distance[end.first][end.second];
    if (shortest_path_sum == MAX)
    {
        return -1;
    }
    
    std::vector<std::pair<int, int>> path;
    std::pair<int, int> current = end;
    path.push_back(current);

    while (current != start)
    {
        current = parent[current.first][current.second];
        path.push_back(current);
    }

    std::reverse(path.begin(), path.end());

    std::cout << "Shortest path: ";
    for (const auto& cell : path)
    {
        std::cout << "(" << cell.first << "," << cell.second << ")";
        if (cell != end)
        {
            std::cout << " -> ";
        }
    }

    std::cout << std::endl;

    return shortest_path_sum;
}

int main()
{
    std::ios::sync_with_stdio(false);
    std::cin.tie(NULL);

    std::string filename = "2D_array_crossing_input.txt";
    std::ifstream input_file(filename);
    if (!input_file)
    {
        std::cerr << "Failed to open file: " << filename << std::endl;
        return 1;
    }

    int rows, columns;
    input_file >> rows >> columns;

    std::vector<std::vector<int>> grid(rows, std::vector<int>(columns));

    for (int i = 0; i < rows; ++i)
    {
        for (int j = 0; j < columns; ++j)
        {
            input_file >> grid[i][j];
        }
    }

    input_file.close();

    std::pair<int, int> start, end;
    std::cout << "Enter the starting point (row col): ";
    std::cin >> start.first >> start.second;
    std::cout << "Enter the ending point (row col): ";
    std::cin >> end.first >> end.second;

    int shortest_path_sum = FindShortestPath(grid, start, end);
    if (shortest_path_sum == -1)
    {
        std::cout << "No valid path found." << std::endl;
    }
    else
    {
        std::cout << "Cost of crossing: " << shortest_path_sum << std::endl;
    }

    return 0;
}
