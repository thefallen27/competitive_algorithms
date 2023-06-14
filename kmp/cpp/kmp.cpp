#include <fstream>
#include <iostream>
#include <string>
#include <vector>

std::vector<int> 
LPSTable(const std::string& pattern) 
{
    std::vector<int> lps(pattern.size(), 0);
    long long int len = 0;
    long long int i = 1;

    while (i < pattern.size()) 
    {
        if (pattern[i] == pattern[len]) 
        {
            lps[i] = ++len;
            ++i;
        }
        else
        {
            if (len != 0)
            {
                len = lps[len - 1];
            }
            else
            {
                lps[i] = 0;
                ++i;
            }
        }
    }

    return lps;
}

void 
KMPSearch(const std::string& pattern, const std::string& filename)
{
    std::ifstream input_file(filename);
    if (!input_file)
    {
        std::cout << "Failed to open the input file." << std::endl;
        return;
    }

    std::vector<int> lps = LPSTable(pattern);
    std::string line;
    int line_index = 1;

    while (std::getline(input_file, line))
    {
        int i = 0;
        long long int j = 0;

        while (i < line.size())
        {
            if (pattern[j] == line[i])
            {
                ++i;
                ++j;
            }

            if (j == pattern.size())
            {
                std::cout << "Pattern found at line " << line_index 
                          << ", index " << (i - j) << std::endl;
                j = lps[j - 1];
            }
            else if (i < line.size() && pattern[j] != line[i])
            {
                if (j != 0)
                {
                    j = lps[j - 1];
                }
                else
                {
                    ++i;
                }
            }
        }

        ++line_index;
    }

    input_file.close();
}

int main()
{
    std::ios::sync_with_stdio(false);
    std::cin.tie(NULL);
    
    std::string pattern;
    std::cout << "Enter the pattern to search: ";
    std::getline(std::cin, pattern);

    std::string filename = "kmp_dna_input.txt";
    
    KMPSearch(pattern, filename);

    return 0;
}
