#include <intrin.h>

long long start = __rdtsc();
// Place your code here that you want to measure
long long finish = __rdtsc();

long long numcycles = finish - start;
double cpufreq = YOUR_CPU_FREQUENCY_GOES_HERE;
double elapsed_time = numcycles / cpufreq * 1000.0;

std::cout << "Elapsed time: " << elapsed_time << "\n";
