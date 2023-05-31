def KnapSack(ks_weight, iweight, iprofit, inumber):
    dp = [0 for i in range(ks_weight + 1)]

    for i in range(1, inumber + 1):
        for w in range(ks_weight, 0, -1):
            if iweight[i - 1] <= w:
                dp[w] = max(dp[w], dp[w - iweight[i - 1]] + iprofit[i - 1])

    return dp[ks_weight]

if __name__ == '__main__':
    with open('knapsack_input.txt', 'r') as f:
        section = 0

        while True:
            item_number = f.readline()

            if item_number == "":
                break
            elif item_number == '\n':
                item_number = f.readline()
   
            item_number = int(item_number)
            total_weight = int(f.readline())
   
            item_profit = []
            item_weight = []
   
            for k in range(item_number):
                ip, iw = map(int, f.readline().split())

                item_profit.append(ip)
                item_weight.append(iw)
                        
            print(f"Maximum profit for section {section}:\n" + str(KnapSack(total_weight, item_weight, item_profit, item_number)))
            section += 1
