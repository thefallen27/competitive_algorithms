The code for Dijkstra's algorithm here is used to read the graph description from a file and then a starting node is asked from the user. The code then calculates the shortest distance from the given input node to all the nodes on the graph.

The graph description in the file is in the form of:
number_of nodes number_of_edges
<starting_node>	<ending_node>	<weight>
<starting_node>	<ending_node>	<weight>
<starting_node>	<ending_node>	<weight>
<starting_node>	<ending_node>	<weight>
...

The number of edges describe the lines that are about to follow.
Also, the algorithm has a line of code commented that once is uncommented, the graph becomes undirected.

In the results file, you will see that for the three nodes given as an example, when the graph is directed there are nodes that do not have a path between them and the starting node. Once the graph is undirectional, this however changes.