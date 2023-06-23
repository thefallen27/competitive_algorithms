#include <fstream>
#include <iostream>
#include <queue>
#include <set>
#include <string>
#include <unordered_map>
#include <vector>

struct TrieNode 
{
    std::unordered_map<char, TrieNode*> children;
    TrieNode* failure;
    bool is_end;

    TrieNode() : failure(nullptr), is_end(false) {}
};

class AhoCorasick 
{
public:
    AhoCorasick() { root_ = new TrieNode(); }

    void addWord(const std::string& word) { InsertWord(word); }

    std::set<std::pair<int, int>> ObscenityOccurence(const std::vector<std::string>& text_lines)
    {
        BuildFailures();
        occurrences_.clear();

        for (int i = 0; i < text_lines.size(); ++i)
        {
            const std::string& line = text_lines[i];
            ObsenityCheck(line, i + 1);
        }

        return occurrences_;
    }

private:
    TrieNode* root_;
    std::set<std::pair<int, int>> occurrences_;

    void InsertWord(const std::string& word) 
    {
        TrieNode* curr = root_;
        std::string lower_case_word = word;
        std::transform(lower_case_word.begin(), lower_case_word.end(), 
                       lower_case_word.begin(), ::tolower);

        for (char c : lower_case_word) 
        {
            if (curr->children.find(c) == curr->children.end())
            {
                curr->children[c] = new TrieNode();
            }

            curr = curr->children[c];
        }

        curr->is_end = true;
    }

    void BuildFailures()
    {
        std::queue<TrieNode*> q;

        for (auto& child : root_->children)
        {
            child.second->failure = root_;
            q.push(child.second);
        }

        while (!q.empty())
        {
            TrieNode* curr = q.front();
            q.pop();

            for (auto& child : curr->children)
            {
                char c = child.first;
                TrieNode* child_node = child.second;
                q.push(child_node);

                TrieNode* failure = curr->failure;
                while (failure && failure->children.find(c) == failure->children.end())
                {
                    failure = failure->failure;
                }

                if (failure)
                {
                    child_node->failure = failure->children[c];
                }
                else
                {
                    child_node->failure = root_;
                }
            }
        }
    }

    void ObsenityCheck(const std::string& text, int line_number)
    {
        TrieNode* curr = root_;
        int len = 0;
        std::string lower_case_text = text;
        std::transform(lower_case_text.begin(), lower_case_text.end(), 
                       lower_case_text.begin(), ::tolower);

        for (int i = 0; i < text.size(); ++i)
        {
            char c = lower_case_text[i];
            while (curr && curr->children.find(c) == curr->children.end())
            {
                curr = curr->failure;
            }

            if (!curr)
            {
                curr = root_;
                len = 0;
            }
            else
            {
                curr = curr->children[c];
                len++;
            }

            if (curr != NULL && curr->is_end) 
            {
                int position = i - len + 2;
                occurrences_.insert(std::make_pair(line_number, position));
            }
        }
    }
};

int main()
{
    std::ios::sync_with_stdio(false);
    std::cin.tie(NULL);
    std::ifstream input_file("aho_corasick_input.txt");

    if (!input_file)
    {
        std::cout << "Failed to open input file." << std::endl;
        return 1;
    }
    
    int n;
    input_file >> n;
    AhoCorasick ac;
    
    for (int i = 0; i < n; i++)
    {
        std::string word;
        input_file >> word;
        ac.addWord(word);
    }

    int m;
    input_file >> m;
    input_file.ignore();

    std::vector<std::string> text_lines;

    for (int i = 0; i < m; i++)
    {
        std::string line;
        std::getline(input_file, line);
        text_lines.push_back(line);
    }

    std::set<std::pair<int, int>> results = ac.ObscenityOccurence(text_lines);

    if (!results.empty())
    {
        for (const auto& result : results)
        {
            std::cout << "Line: " << result.first << " position: " << result.second << std::endl;
        }
    }
    else
    {
        std::cout << "Passed" << std::endl;
    }

    return 0;
}
