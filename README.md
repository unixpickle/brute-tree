# brute-tree

A distributed brute-force decision tree solver for classification problems. Aim is to find small decision trees that get high training accuracy on datasets like MNIST and CIFAR-10.

# Design

The system has a *server* and one or more *workers*. Each worker requests batches of work from the server, does the work, and returns the results to the server. If the work is not finished within a certain time window, the server will reassign the work to a different worker (to deal with node failures).
