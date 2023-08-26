There are three input files for this algorithm. All of them describe graphs with the first line declaring the number of nodes and the number of edges. Each subsequent line describes an edge by stating the node it begins from and the node it connects to.

The algorithm we implemented here uses an adjacency list and colours the various nodes using the two-colouring approach to determine the bipartiteness of the given graph.

The first input file describes a graph that is not bipartite, while the second one is of a bipartite graph. The third input is what we call a Mongolian Tent graph. Mongolian Tent graphs are bipartite by nature and they are a graceful. A graceful graph has m edges in which the nodes are labeled with a subset of distinct nonnegative integers from 0 to m and the graph edges are labeled with the absolute differences between node values. This is what we call graceful labeling and constitutes a graceful graph.

The Mongolian Tent graph used here is described by M(4,7). This means that it has a root, 4 rows, and 7 columns. To see how it looks like, have a look at the JPEG in this repository.
