def LPSTable(pattern):
    lps = [0] * len(pattern)
    length = 0
    i = 1

    while i < len(pattern):
        if pattern[i] == pattern[length]:
            length += 1
            lps[i] = length
            i += 1
        else:
            if length != 0:
                length = lps[length - 1]
            else:
                lps[i] = 0
                i += 1

    return lps

def KMPSearch(pattern, filename):
    try:
        with open(filename, 'r') as input_file:
            lps = LPSTable(pattern)
            line_index = 1

            for line in input_file:
                i, j = 0, 0

                while i < len(line):
                    if pattern[j] == line[i]:
                        i += 1
                        j += 1

                    if j == len(pattern):
                        print(f"Pattern found at line {line_index}, index {i - j}")
                        j = lps[j - 1]
                    elif i < len(line) and pattern[j] != line[i]:
                        if j != 0:
                            j = lps[j - 1]
                        else:
                            i += 1

                line_index += 1
    except FileNotFoundError:
        print("Failed to open the input file.")

if __name__ == "__main__":
    pattern = input("Enter the pattern to search: ")
    filename = "kmp_lorem_ipsum_input.txt"
    KMPSearch(pattern, filename)