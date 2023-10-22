As we did with other algorithms, we chose for this one to solve a problem given to a competition. The following problem was handed out to participants in the Benelux Algorithm Programming Contest of 2017. The problem's name was Easter Eggs. Here is the description:

Easter is coming and the Easter Bunny decided to organise a chocolate egg hunt for the children. He will hide two types of eggs: blue milk chocolate and red dark chocolate.

In the field there are some redberry and some blueberry plants where the Easter Bunny could hide the eggs. Red eggs should be hidden in a redberry plant and blue eggs in a blueberry plant.

The local government has issued a permit for the event, under the condition that exactly N eggs are hidden. As they do not pay for the dental care plans of the local children, the Easter Bunny gets to decide himself how many eggs to hide of each colour.

According to the yearly tradition, there is a big reward for the first child to find both a red and a blue egg. In order to make the hunt as challenging as possible, the Easter Bunny wants to maximise the minimum distance between a red and a blue egg. To keep things fair, he will hide at most one egg in each plant. Your task is to write a program to help him accomplish his goal.

==============================================

After reading the problem you can see that we have a bipartite graph that consists of the plants, then we have the eggs that going to match their colour with the colour of the plant the bunny will hide them in (which is the matching process), and finally we need the maximum minimum distance which leads us to Konig's theorem!