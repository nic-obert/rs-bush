use std::ptr::null_mut;
use std::collections::VecDeque;


pub struct BushNodeItemIterRight<'a, T> {

    node: Option<&'a BushNode<T>>

}


pub struct BushNodeItemIterLeft<'a, T> {

    node: Option<&'a BushNode<T>>

}


impl<'a, T> Iterator for BushNodeItemIterRight<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(node) = self.node {
            self.node = if node.right.is_null() {
                None
            } else {
                unsafe {
                    Some(&(*node.right))
                }
            };
            Some(&node.item)
        } else {
            None
        }
    }
}


impl<'a, T> Iterator for BushNodeItemIterLeft<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(node) = self.node {
            self.node = if node.left.is_null() {
                None
            } else {
                unsafe {
                    Some(&(*node.left))
                }
            };
            Some(&node.item)
        } else {
            None
        }
    }
}


pub struct BushNodeIterLeft<'a, T> {

    node: Option<&'a BushNode<T>>

}


impl<'a, T> Iterator for BushNodeIterLeft<'a, T> {
    type Item = &'a BushNode<T>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(node) = self.node {
            self.node = if node.left.is_null() {
                None
            } else {
                unsafe {
                    Some(&(*node.left))
                }
            };
            Some(node)
        } else {
            None
        }
    }
}


pub struct BushNodeIterRight<'a, T> {

    node: Option<&'a BushNode<T>>

}


impl<'a, T> Iterator for BushNodeIterRight<'a, T> {
    type Item = &'a BushNode<T>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(node) = self.node {
            self.node = if node.right.is_null() {
                None
            } else {
                unsafe {
                    Some(&(*node.right))
                }
            };
            Some(node)
        } else {
            None
        }
    }
}


pub struct BushNode<T> {

    left: *mut BushNode<T>,
    right: *mut BushNode<T>,

    pub children: Option<Bush<T>>,

    pub item: T

}


impl<T> BushNode<T> {

    /// Create a new bush node
    pub fn new(item: T, left: *mut BushNode<T>) -> BushNode<T> {
        Self {
            left,
            right: null_mut(),
            children: None,
            item
        }
    }


    /// Insert the given slice to the left while preserving the links
    pub fn insert_slice_left(&mut self, start_node: &mut BushNode<T>, end_node: &mut BushNode<T>) {
        
        if !self.left.is_null() {
            unsafe {
                (*self.left).right = start_node;
            }
        }

        start_node.left = self.left;
        end_node.right = self;

        self.left = end_node;
    }


    /// Insert the given slice to the right while preserving the links
    pub fn insert_slice_right(&mut self, start_node: &mut BushNode<T>, end_node: &mut BushNode<T>) {

        if !self.right.is_null() {
            unsafe {
                (*self.right).left = end_node;
            }
        }

        end_node.right = self.right;
        start_node.left = self;

        self.right = start_node;
    }


    /// Insert the given node to the left while preserving the links
    pub fn insert_left_node(&mut self, node: &mut BushNode<T>) {
        node.right = self;

        if !self.left.is_null() {
            unsafe {
                (*self.left).right = node;
            }
        } 

        node.left = self.left;

        self.left = node;
    }


    /// Insert the given node to the right while preserving the links
    pub fn insert_right_node(&mut self, node: &mut BushNode<T>) {
        node.left = self;

        if !self.right.is_null() {
            unsafe {
                (*self.right).left = node;
            }
        }

        node.right = self.right;

        self.right = node;
    }


    /// Get the node to the left, if any
    pub fn left_node(&self) -> Option<&BushNode<T>> {
        if self.left.is_null() {
            None
        } else {
            unsafe {
                Some(&*self.left)
            }
        }
    }


    /// Get the node to the right, if any
    pub fn right_node(&self) -> Option<&BushNode<T>> {
        if self.right.is_null() {
            None
        } else {
            unsafe {
                Some(&*self.right)
            }
        }
    }


    /// Get the item to the left, if any
    pub fn left_item(&self) -> Option<&T> {
        if self.left.is_null() {
            None
        } else {
            unsafe {
                Some(&(*self.left).item)
            }
        }
    }


    /// Get the item to the right, if any
    pub fn right_item(&self) -> Option<&T> {
        if self.right.is_null() {
            None
        } else {
            unsafe {
                Some(&(*self.right).item)
            }
        }
    }


    /// Get an iterator over the items to the left
    pub fn iter_item_left(&self) -> BushNodeItemIterLeft<'_, T> {
        BushNodeItemIterLeft { 
            node: Some(self)
        }
    }


    /// Get an iterator over the items to the right
    pub fn iter_item_right(&self) -> BushNodeItemIterRight<'_, T> {
        BushNodeItemIterRight { 
            node: Some(self)
        }
    }


    /// Get an iterator over the nodes to the left
    pub fn iter_left(&self) -> BushNodeIterLeft<'_, T> {
        BushNodeIterLeft { 
            node: Some(self)
        }
    }


    /// Get an iterator over the nodes to the right
    pub fn iter_right(&self) -> BushNodeIterRight<'_, T> {
        BushNodeIterRight { 
            node: Some(self)
        }
    }

}


pub struct Bush<T> {

    first: *mut BushNode<T>,
    last: *mut BushNode<T>

}


impl<T> Bush<T> {

    /// Create a new empty bush
    pub fn new() -> Bush<T> {
        Self::default()
    }


    /// Return the number of nodes in the bush's top layer
    pub fn top_layer_length(&self) -> usize {
        self.iter_nodes().count()
    }


    pub fn total_node_count(&self) -> usize {
        todo!()
    }


    /// Get the last item of the bush's top layer
    pub fn last_item(&self) -> Option<&T> {
        if self.last.is_null() {
            None
        } else {
            unsafe {
                Some(&(*self.last).item)
            }
        }
    }


    /// Get the last node if the bush's top layer
    pub fn last_node(&self) -> Option<&BushNode<T>> {
        if self.last.is_null() {
            None
        } else {
            unsafe {
                Some(&*self.last)
            }
        }
    }


    /// Get the first item of the bush's top layer
    pub fn first_item(&self) -> Option<&T> {
        if self.first.is_null() {
            None
        } else {
            unsafe {
                Some(&(*self.first).item)
            }
        }
    }


    /// Get the first node of the bush's top layer
    pub fn first_node(&self) -> Option<&BushNode<T>> {
        if self.first.is_null() {
            None
        } else {
            unsafe {
                Some(&*self.first)
            }
        }
    }


    /// Get the nth node of the bush's top layer
    pub fn nth_node(&self, i: usize) -> Option<&BushNode<T>> {
        self.iter_nodes().nth(i)
    }


    /// Get the nth item of the bush's top layer
    pub fn nth_item(&self, i: usize) -> Option<&T> {
        self.iter_items().nth(i)
    }


    /// Append a new node to the bush's top layer
    pub fn append(&mut self, item: T) {

        let node = Box::leak(Box::new(BushNode::new(item, self.last)));

        if self.last.is_null() {
            // Bush is empty
            self.first = node;
            self.last = node;
        } else {
            // Bush is not empty
            unsafe {
                (*self.last).right = node;
            }
            self.last = node;
        }
    }


    /// Get an iterator over the items of the bush's top layer
    pub fn iter_items(&self) -> BushNodeItemIterRight<'_, T> {
        BushNodeItemIterRight {
            node: if self.first.is_null() {
                None
            } else {
                Some(unsafe { &*self.first })
            }
        }
    }


    /// Get an iterator over the nodes of the bush's top layer
    pub fn iter_nodes(&self) -> BushNodeIterRight<'_, T> {
        BushNodeIterRight {
            node: if self.first.is_null() {
                None
            } else {
                Some(unsafe { &*self.first })
            }
        }
    }


    /// Extrat the given node and its branches, assuming that the node is in the bush's top layer
    pub fn extract_node(&mut self, node: &BushNode<T>) -> &mut BushNode<T> {
        let node_ptr = node as *const BushNode<T> as *mut BushNode<T>;

        if node_ptr != self.first {
            let left_node = node.left;
            // Assume the left node is not null since the current node is not the first
            unsafe {
                (*left_node).right = node.right
            }
        }

        if node_ptr != self.last {
            let right_node = node.right;
            // Assume the right node is not null since the current node is not the last
            unsafe {
                (*right_node).left = node.left;
            }
        }

        unsafe { &mut *node_ptr }
    }


    /// Extract a slice of the bush and the relative branches into a new bush, assumimg the nodes are n the bush's top layer
    pub fn extract_slice(&mut self, start_node: &BushNode<T>, end_node: &BushNode<T>) -> Bush<T> {
        let start_ptr = start_node as *const BushNode<T> as *mut BushNode<T>;
        let end_ptr = end_node as *const BushNode<T> as *mut BushNode<T>;

        if start_ptr != self.first {
            let left_node = start_node.left;
            // Assume the left node is not null
            unsafe {
                (*left_node).right = end_node.right;
            }
        }

        if end_ptr != self.last {
            let right_node = end_node.right;
            // Assume the right node is nt null
            unsafe {
                (*right_node).left = start_node.left;
            }
        }

        Bush {
            first: start_ptr,
            last: end_ptr
        }
    }


    /// Recursively flatten the bush into the top layer
    pub fn flatten(&mut self) {
        
        let mut node_ptr = self.first;
        
        while !node_ptr.is_null() {

            let node = unsafe { &mut *node_ptr };
            node_ptr = node.right;

            if let Some(mut children) = node.children.take() {
                children.flatten();
                if let Some((start, end)) = children.extract() {
                    node.insert_slice_right(unsafe { &mut *start }, unsafe { &mut *end });
                }
            }
        }

    }


    /// Get a breadth first search iterator over the bush
    pub fn bfs(&self) -> BFSIter<T> {
        BFSIter {
            nodes: if self.first.is_null() {
                VecDeque::new()
            } else {
                VecDeque::from(vec![unsafe { &*self.first }])
            }
        }
    }


    /// Get a depth first search iterator over the bush
    pub fn dfs(&self) -> DFSIter<T> {
        DFSIter {
            nodes: if self.first.is_null() {
                VecDeque::new()
            } else {
                VecDeque::from(vec![unsafe { &*self.first }])
            }
        }
    }


    /// Return the first and last node of the bush's top layer, consuming the bush
    pub fn extract(self) -> Option<(*mut BushNode<T>, *mut BushNode<T>)> {
        if self.first.is_null() {
            None
        } else {
            Some((self.first, self.last))
        }
    }


}


pub struct BFSIter<'a, T> {

    nodes: VecDeque<&'a BushNode<T>>,

}


impl<'a, T> Iterator for BFSIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        
        if let Some(node) = self.nodes.pop_front() {

            // Push the same-layer node on the front to give it priority
            if let Some(node) = node.right_node() {
                self.nodes.push_front(node);
            }

            // Push lower-layer nodes on the back for lower priority
            if let Some(children) = &node.children {
                if let Some(first_node) = children.first_node() {
                    self.nodes.push_back(first_node);
                }
            }

            Some(&node.item)

        } else {
            // There are no more nodes to search
            None
        }
    }
}


pub struct DFSIter<'a, T> {

    nodes: VecDeque<&'a BushNode<T>>,

}


impl<'a, T> Iterator for DFSIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {

        if let Some(node) = self.nodes.pop_front() {

            // Push the same-layer nodes before children nodes to give children priority
            if let Some(right) = node.right_node() {
                self.nodes.push_front(right);
            }

            if let Some(children) = &node.children {
                if let Some(first_node) = children.first_node() {
                    self.nodes.push_front(first_node);
                }
            }

            Some(&node.item)
        } else {
            // There are no more nodes to search
            None
        }
    }
}


impl<T> Default for Bush<T> {
    fn default() -> Self {
        Self {
            first: null_mut(),
            last: null_mut()
        }
    }
}


impl<T> Drop for Bush<T> {
    fn drop(&mut self) {
        // Recursively drop nodes
        let mut node = self.first;
        while !node.is_null() {
            let next_node = unsafe { (*node).right };
            unsafe { Box::from_raw(node) };
            node = next_node;
        }
    }
}

