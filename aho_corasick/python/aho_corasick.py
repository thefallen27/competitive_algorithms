import collections
import string

class TrieNode:
    def __init__(self):
        self.children = {}
        self.failure = None
        self.is_end = False

class AhoCorasick:
    def __init__(self):
        self.root = TrieNode()
        self.occurrences = set()

    def add_word(self, word):
        word = word.lower()
        node = self.root
        for char in word:
            if char not in node.children:
                node.children[char] = TrieNode()
            node = node.children[char]
        node.is_end = True

    def build_failures(self):
        queue = collections.deque()
        for char, child in self.root.children.items():
            child.failure = self.root
            queue.append(child)

        while queue:
            node = queue.popleft()
            for char, child in node.children.items():
                queue.append(child)
                failure = node.failure
                while failure and char not in failure.children:
                    failure = failure.failure
                if failure:
                    child.failure = failure.children[char]
                else:
                    child.failure = self.root

    def obscenity_occurrence(self, text_lines):
        self.build_failures()
        self.occurrences.clear()

        for line_num, line in enumerate(text_lines, start=1):
            line = line.lower()
            node = self.root
            length = 0

            for i, char in enumerate(line, start=1):
                while node and char not in node.children:
                    node = node.failure
                    length = 0
                if not node:
                    node = self.root
                    length = 0
                else:
                    node = node.children[char]
                    length += 1
                if node and node.is_end:
                    position = i - length + 1
                    self.occurrences.add((line_num, position))

        return self.occurrences

if __name__ == "__main__":
    try:
        with open("aho_corasick_input.txt", "r") as input_file:
            n = int(input_file.readline().strip())
            ac = AhoCorasick()
            for _ in range(n):
                word = input_file.readline().strip()
                ac.add_word(word)

            m = int(input_file.readline().strip())
            text_lines = [input_file.readline().strip() for _ in range(m)]
    except FileNotFoundError:
        print("Failed to open input file.")
        exit()

    results = ac.obscenity_occurrence(text_lines)

    if results:
        for line_num, position in results:
            print(f"Line: {line_num}, position: {position}")
    else:
        print("Passed")
