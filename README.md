# brute-tree

A distributed brute-force decision tree solver for classification problems. Aim is to find small decision trees that get high training accuracy on datasets like MNIST and CIFAR-10.

# Design

The system has a *server* and one or more *workers*. Each worker repeatedly evaluates random trees and periodically sends the best tree it has found to the server.

# Results

This table includes the training accuracy for different versions of this algorithm. In all cases, the algorithm was run for around a day on >100 cores. I will fill in this table as I run more experiments.

|                    | Depth 5 | Depth 6 |
|--------------------|---------|---------|
| Random Search      | 0.46    | --      |
| Basic Evolution    | 0.7732  | 0.8115  |
| Sub-tree Evolution | 0.7835  | 0.8203  |
