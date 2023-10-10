# rs-bush

Implementation of the bush data structure in Rust.

The bush data structure is an hybrid of a doubly-linked list and a tree. It allows for fast insertion and removal of elements, as well as creating 2-dimensional graphs.

The bush is basically a doubly-linked list of nodes, each of which can have a child bush. The bush is a tree, but it is not a binary tree. Each node can have any number of children.

