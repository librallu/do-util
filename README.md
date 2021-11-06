# Discrete Optimization utility



## Set data-structures

Various data-structures to maintain efficiently sets

 - [X] **Sparse-set:** Maintains sets of positive integers. Allows for O(1) insertion, deletion, counts, delete all but one element. This data-structure is expensive to create, but the operations are very fast. See [this article](https://hal.archives-ouvertes.fr/hal-01339250/document) for more information.

### Benchmarks



## Sub-set/Super-set queries

Allows performing quick sub-set or super-set queries

 - [ ] **Set-trie.** See [this article](https://hal.inria.fr/hal-01506780/document) for more information.

### Benchmarks



## Pareto priority-queues

Data-structures for quick insertion/removal/find-minimum/dominance-checks on an n-dimensional pareto front.
Heavily inspired from the excellent [pareto library for Python and C++](https://github.com/alandefreitas/pareto) [[1]](#1). Each Pareto front stores elements such that no element dominates another. The main
difference is that the data-structures in this crate allow to find the minimum element quickly.
Moreover, using this crate, it is possible to define more general dominance rules in addition of
the dimension dominance.

 - [X] **List Pareto front**
 - [X] **Naive kd-tree**
 - [ ] **Quad-tree**
 - [ ] **Bucket list**


### Benchmarks



## Crate status

This crate is in active development, Pull requests welcome :)

## References

<a id="1">[1]</a> Alan Freitas,
Efficient user-oriented Pareto fronts and Pareto archives based on spatial data structures,
Swarm and Evolutionary Computation,
Volume 65,
2021,
100915,
ISSN 2210-6502,
https://doi.org/10.1016/j.swevo.2021.100915.
(https://www.sciencedirect.com/science/article/pii/S2210650221000766)
