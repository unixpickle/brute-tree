# brute-tree

A distributed brute-force decision tree solver for classification problems. Aim is to find small decision trees that get high training accuracy on datasets like MNIST and CIFAR-10.

# Design

The system has a *server* and one or more *workers*. Each worker repeatedly evaluates random trees and periodically sends the best tree it has found to the server.
