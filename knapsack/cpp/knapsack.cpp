#include <iostream>
#include <fstream>
#include <vector>
#include <algorithm>

int 
KnapSack(int ks_weight, const std::vector<int>& iweight, const std::vector<int>& iprofit)
{
    std::vector<int> max_value(ks_weight + 1, 0);

    for (int i = 0; i < iweight.size(); ++i)
    {
        for (long long int w = ks_weight; w >= iweight[i]; --w) 
        {
            max_value[w] = std::max(max_value[w], max_value[w - iweight[i]] + iprofit[i]);
        }
    }
    
    return max_value[ks_weight];
}

int main()
{
    std::ios::sync_with_stdio(false);
    std::cin.tie(NULL);

    std::ifstream input_file("input.txt");

    if (!input_file) 
    {
        std::cerr << "Error opening file!" << "\n";
        return 1;
    }

    int item_number, total_weight;
    int section = 0;

    while (input_file >> item_number >> total_weight) 
    {
        std::vector<int> profit(item_number);
        std::vector<int> weight(item_number);

        for (int i = 0; i < item_number; ++i) 
        {
            int item_profit, item_weight;
            input_file >> item_profit >> item_weight;
            profit[i] = item_profit;
            weight[i] = item_weight;
        }

        std::cout << "Maximum profit for section " << section << ":" << "\n";
        std::cout << KnapSack(total_weight, weight, profit) << "\n";
        section++;
    }

    input_file.close();
    return 0;
}
