#include <algorithm>
#include <fstream>
#include <iostream>
#include <stack>
#include <vector>

struct Point
{
    int x, y;
};


Point
NextToTop(std::stack<Point>& sp)
{
    Point p = sp.top();
    sp.pop();
    Point res = sp.top();
    sp.push(p);

    return res;
}

int
SquareDistance(Point p1, Point p2)
{
    return (p1.x - p2.x) * (p1.x - p2.x) + (p1.y - p2.y) * (p1.y - p2.y);
}

// Compare points based on their polar angle
int
Orientation(Point p, Point q, Point r)
{
    int val = (q.y - p.y) * (r.x - q.x) - (q.x - p.x) * (r.y - q.y);

    if (val == 0)
    {
        return 0;
    }

    return (val > 0) ? 1 : 2;
}

//Covnex hull is calculated using Graham's scan algorithm
std::vector<Point>
ConvexHull(std::vector<Point>& points)
{
    long long int n = points.size();
    if (n < 3)
    {
        return {};
    }

    // Lowest point on the graph (bottom) placed in the first position
    int y_minimum = points[0].y, minimum_index = 0;
    for (int i = 1; i < n; i++)
    {
        int y = points[i].y;
        if ((y < y_minimum) || (y_minimum == y && points[i].x < points[minimum_index].x))
        {
            y_minimum = points[i].y;
            minimum_index = i;
        }
    }

    std::swap(points[0], points[minimum_index]);

    // Sort remaining points based on polar angle in counterclockwise order
    Point p0 = points[0];
    std::sort(points.begin() + 1, points.end(), [&](const Point& p1, const Point& p2)
		{ int orientation = Orientation(p0, p1, p2); 
			if (orientation == 0) { return (SquareDistance(p0, p2) >= SquareDistance(p0, p1)); }
				return (orientation == 2); });

    // Removing any collinear points
    int m = 1;
    for (long long int i = 1; i < n; i++)
    {
        while (i < n - 1 && Orientation(p0, points[i], points[i + 1]) == 0)
        {
            i++;
        }

        points[m] = points[i];
        m++;
    }

    if (m < 3)
    {
        return {};
    }

    std::stack<Point> stack_point;
    stack_point.push(points[0]);
    stack_point.push(points[1]);

    for (int i = 3; i < m; i++)
    {
        while (Orientation(NextToTop(stack_point), stack_point.top(), points[i]) != 2)
        {
            stack_point.pop();
        }

        stack_point.push(points[i]);
    }

    std::vector<Point> hull;
    while (!stack_point.empty())
    {
        hull.push_back(stack_point.top());
        stack_point.pop();
    }

    return hull;
}

int main()
{
    std::ios::sync_with_stdio(false);
    std::cin.tie(NULL);

    std::ifstream input_file("convex_hull_input.txt");
    if (!input_file)
    {
        std::cout << "Failed to open input file." << std::endl;
        return 1;
    }

    int number_of_points;
    input_file >> number_of_points;

    std::vector<Point> points(number_of_points);
    for (int i = 0; i < number_of_points; i++)
    {
        input_file >> points[i].x >> points[i].y;
    }

    input_file.close();

    std::vector<Point> convex_hull_points = ConvexHull(points);

    std::cout << "Convex Hull points:" << std::endl;
    for (const auto& p : convex_hull_points)
    {
        std::cout << p.x << " " << p.y << std::endl;
    }

    return 0;
}
