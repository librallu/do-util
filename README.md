# Discrete Optimization utility

Implements various useful data-structures for discrete optimization. This crate is in active development, Pull requests welcome :).



## Set data-structures

Various data-structures to maintain efficiently sets

 - [X] **Sparse-set:** Maintains sets of positive integers. Allows for O(1) insertion, deletion, counts, delete all but one element. This data-structure is expensive to create, but the operations are very fast. See [this article](https://hal.archives-ouvertes.fr/hal-01339250/document) for more information.

### Benchmarks



## Sub-set/Super-set queries

Allows performing quick sub-set or super-set queries.

 - [X] **List** Simple naive list storage. Iterates over the whole list to find sub-sets/super-sets
 - [X] **Set-trie** See [this article](https://hal.inria.fr/hal-01506780/document) for more information.
       It is fast for sub-set queries, slower for super-set queries. Is efficient if the number of elements in sets is small.
 - [ ] **HAT-trie** See [this article](https://ieeexplore.ieee.org/document/8478414) for more information.

### Benchmarks


## Pareto priority-queues

Data-structures for quick insertion/removal/find-minimum/dominance-checks on an n-dimensional pareto front. Each element also provides a "guide" value that is used for minimum (resp. maximum) extraction.

Heavily inspired from the excellent [pareto library for Python and C++](https://github.com/alandefreitas/pareto) [[1]](#1). Each Pareto front stores elements such that no element dominates another. The main
difference is that the data-structures in this crate allow to find the minimum element quickly.
Moreover, using this crate, it is possible to define more general dominance rules in addition of
the dimension dominance.

 - [X] **List Pareto front:** Simple data-structure that simply stores the elements using a vector. This data-structure is straightforward, and usually works fine for small 
 - [X] **Kd-tree:** Data-structure in which each node contains an element and divides the space into 2 parts. This data-structure is efficient for many points.
 - [ ] **Point-region-tree:** Data-structure in which each node divides the space into 2**d subregions. This data-structure is efficient for many points, but requires an initial lower/upper bound on the dimensions.
 - [ ] **R-tree:** Data-structure in which elements are stored in bounding boxes. Bounding boxes may intersect.
 - [ ] **R\*-tree:**

### Benchmarks

Random n-dimensional points.



## Roadmap

- [ ] refactor pareto-pq to priority queue
- [ ] add guide in list pareto
- [ ] add guide in kd-tree pareto


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
