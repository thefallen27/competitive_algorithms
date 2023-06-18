For this algorithm, we chose instead of just showing the algorithm 
in a program, to solve an actual problem from a competition. We took 
the problem as presented here: https://acm.timus.ru/problem.aspx?space=1&num=1269
and we developed the code for it. Here is the description of the problem:

There is a problem to check messages of web-board visitors for the obscene words. 
Your elder colleagues commit this problem to you. You are to write a program, 
which check if there is at least one obscene word from the given list in the 
given text as a substring.

Input
The first line consists of integer n (1 ≤ n ≤ 10000) — an amount of words. 
The next n lines contain the list of words that we can’t allow to use in our
well-educated society. A word may contain any symbol but the ones with 
codes 0, 10 and 13. The length of each word doesn’t exceed 10000 symbols.
The total list of words doesn’t exceed 100 KB. Then there is an integer 
m — the number of lines of the text. A size of the text doesn’t exceed 900 KB.

Output
The number of line and the number of position separated with a space, 
where an obscene word occurs for the first time. If there are no 
obscene words, output “Passed”.

The sample give is the following:

Input:
5
dear
sweetie
angel
dream
baby
8
Had I the heavens' embroidered cloths, 
Enwrought with golden and silver light, 
The blue and the dim and the dark cloths 
Of night and light and the half-light, 
I would spread the cloths under your feet: 
But I, being poor, have only my dreams; 
I have spread my dreams under your feet; 
Tread softly because you tread on my dreams.

Output:
6 33

Notes:
1. We improved a bit on the solution to make it a bit more generic. Our code prints
   all the occurences of the obscene words; it doesn't stop on the first occurence.
2. The text given above (the poem) is "Aedh Wishes for the Cloths of Heaven", by W. B. Yeats,
   one of the most prominent figures of 20th century literature, and a founder of the Abbey Theatre.
