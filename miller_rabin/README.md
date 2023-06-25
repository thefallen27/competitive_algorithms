The code here asks the user for a number and finds its prime factors using the Miller-Rabin primality test.
Please keep in mind that Miller-Rabin is a probabilistic algorithm, and it may not always yield the correct
prime factors. That being said, it is a better algorithm in determining the primality of a number than other
algorithms. In order to avoid false positives (or strong pseudoprimes as they are called), you can increase
the 'k' value that we provide in our code as the second argument to the Miller-Rabin function, as shown below:

						bool MillerRabin(uint64_t number, int k = 500)

A general rule of thumb is that k = 5 is a good number for repetitions and it will yield satisfying results.
To increase accuracy, we set k to 500. Please bear in mind that increasing k more will increase accuracy (albeit
it is very accurate as is) but it will also increase execution time.
