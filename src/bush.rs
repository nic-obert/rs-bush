use std::{ptr::null_mut, collections::VecDeque};


pub type IterItems<T, It> = std::iter::Map<It, fn(&BushNode<T>) -> &T>;
pub type IterItemsMut<T, It> = std::iter::Map<It, fn(&mut BushNode<T>) -> &mut T>;

pub type BushSlice<T> = (Box<BushNode<T>>, Box<BushNode<T>>);


pub struct NodeHandle<T> (*const BushNode<T>);

impl<T> NodeHandle<T> {

    #[inline(always)]
    pub const fn as_ptr(&self) -> *const BushNode<T> {
        self.0
    }


    #[inline(always)]
    pub const fn as_ref(&self) -> &BushNode<T> {
        unsafe { &*self.0 }
    }


    pub const fn clone(&self) -> NodeHandle<T> {
        NodeHandle(self.0)
    }

}


pub struct BushNodeItemIterRight<'a, T> {

    node: Option<&'a BushNode<T>>

}


pub struct BushNodeItemIterRightMut<'a, T> {

    node: Option<&'a mut BushNode<T>>

}


pub struct BushNodeItemIterLeft<'a, T> {

    node: Option<&'a BushNode<T>>

}


pub struct BushNodeItemIterLeftMut<'a, T> {

    node: Option<&'a mut BushNode<T>>

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


impl<'a, T> Iterator for BushNodeItemIterRightMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(node) = self.node.take() {
            if !node.right.is_null() {
                unsafe {
                    self.node = Some(&mut (*node.right))
                }
            };
            Some(&mut node.item)
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


impl<'a, T> Iterator for BushNodeItemIterLeftMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(node) = self.node.take() {
            if !node.left.is_null() {
                unsafe {
                    self.node = Some(&mut (*node.left))
                }
            };
            Some(&mut node.item)
        } else {
            None
        }
    }
}


pub struct BushNodeIterLeft<'a, T> {

    node: Option<&'a BushNode<T>>

}


pub struct BushNodeIterLeftMut<'a, T> {

    node: Option<&'a mut BushNode<T>>

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


impl<'a, T> Iterator for BushNodeIterLeftMut<'a, T> {
    type Item = &'a mut BushNode<T>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(node) = self.node.take() {
            if !node.left.is_null() {
                unsafe {
                    self.node = Some(&mut (*node.left))
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


pub struct BushNodeIterRightMut<'a, T> {

    node: Option<&'a mut BushNode<T>>

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


impl<'a, T> Iterator for BushNodeIterRightMut<'a, T> {
    type Item = &'a mut BushNode<T>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(node) = self.node.take() {
            if !node.right.is_null() {
                unsafe {
                    self.node = Some(&mut (*node.right))
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


    pub fn into_handle(&self) -> NodeHandle<T> {
        NodeHandle(self as *const BushNode<T>)
    }


    /// Insert the given slice to the left while preserving the links
    pub fn insert_slice_left(&mut self, slice: BushSlice<T>) {
        let start_node = Box::leak(slice.0);
        let end_node = Box::leak(slice.1);
        
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
    pub fn insert_slice_right(&mut self, slice: BushSlice<T>) {

        let start_node = Box::leak(slice.0);
        let end_node = Box::leak(slice.1);

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
    pub fn insert_left_node(&mut self, node: Box<BushNode<T>>) {
        let node = Box::leak(node);

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
    pub fn insert_right_node(&mut self, node: Box<BushNode<T>>) {
        let node = Box::leak(node);

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


    /// Get the node to the left, if any
    pub fn left_node_mut(&self) -> Option<&mut BushNode<T>> {
        if self.left.is_null() {
            None
        } else {
            unsafe {
                Some(&mut *self.left)
            }
        }
    }


    /// Get the node to the right, if any
    pub fn right_node_mut(&self) -> Option<&mut BushNode<T>> {
        if self.right.is_null() {
            None
        } else {
            unsafe {
                Some(&mut *self.right)
            }
        }
    }


    /// Get the item to the left, if any
    pub fn left_item(&self) -> Option<&T> {
        self.left_node().map(|node| &node.item)
    }


    /// Get the item to the right, if any
    pub fn right_item(&self) -> Option<&T> {
        self.right_node().map(|node| &node.item)
    }


    /// Get an iterator over the items to the left
    pub fn iter_items_left(&self) -> IterItems<T, BushNodeIterLeft<T>> {
        self.iter_nodes_left().map(|node| &node.item)
    }


    /// Get an iterator over the items to the right
    pub fn iter_items_right(&self) -> IterItems<T, BushNodeIterRight<T>> {
        self.iter_nodes_right().map(|node| &node.item)
    }


    /// Get an iterator over the nodes to the left
    pub fn iter_nodes_left(&self) -> BushNodeIterLeft<'_, T> {
        BushNodeIterLeft { 
            node: Some(self)
        }
    }


    /// Get an iterator over the nodes to the right
    pub fn iter_nodes_right(&self) -> BushNodeIterRight<'_, T> {
        BushNodeIterRight { 
            node: Some(self)
        }
    }


    pub fn bfs_nodes(&self) -> BFSIter<T> {
        BFSIter {
            nodes: self.children.as_ref().map(
                |children|
            {
                let mut nodes = VecDeque::new();
                if let Some(first_node) = children.first_node() {
                    nodes.push_back(first_node);
                }
                nodes
            }).unwrap_or_default()
        }
    }


    pub fn dfs_nodes(&self) -> DFSIter<T> {
        DFSIter {
            nodes: self.children.as_ref().map(
                |children|
            {
                let mut nodes = VecDeque::new();
                if let Some(first_node) = children.first_node() {
                    nodes.push_back(first_node);
                }
                nodes
            }).unwrap_or_default()
        }
    }


    pub fn bfs_items(&self) -> IterItems<T, BFSIter<T>> {
        self.bfs_nodes().map(|node| &node.item)
    }


    pub fn dfs_items(&self) -> IterItems<T, DFSIter<T>> {
        self.dfs_nodes().map(|node| &node.item)
    }

}


pub struct Bush<T> {

    first: *mut BushNode<T>,
    last: *mut BushNode<T>,

}


impl<T> Bush<T> {

    /// Create a new empty bush
    pub fn new() -> Bush<T> {
        Self::default()
    }


    pub fn is_empty(&self) -> bool {
        self.first.is_null()
    }


    /// Return the number of nodes in the bush's top layer
    pub fn top_layer_length(&self) -> usize {
        self.iter_nodes().count()
    }


    /// Return the total number of nodes in the bush
    pub fn total_node_count(&self) -> usize {
        self.iter_nodes().map(
            |node| {
                let mut count = 1;

                if let Some(children) = &node.children {
                    count += children.total_node_count();
                }

                count
            }
        ).sum()
    }


    /// Get the last item of the bush's top layer
    pub fn last_item(&self) -> Option<&T> {
        self.last_node().map(|node| &node.item)
    }


    /// Get the last item of the bush's top layer
    pub fn last_item_mut(&self) -> Option<&mut T> {
        self.last_node_mut().map(|node| &mut node.item)
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


    /// Get the last node if the bush's top layer
    pub fn last_node_mut(&self) -> Option<&mut BushNode<T>> {
        if self.last.is_null() {
            None
        } else {
            unsafe {
                Some(&mut *self.last)
            }
        }
    }


    /// Get the first item of the bush's top layer
    pub fn first_item(&self) -> Option<&T> {
        self.first_node().map(|node| &node.item)
    }


    /// Get the first item of the bush's top layer
    pub fn first_item_mut(&self) -> Option<&mut T> {
        self.first_node_mut().map(|node| &mut node.item)
    }


    pub fn first_node_handle(&self) -> Option<NodeHandle<T>> {
        if self.first.is_null() {
            None
        } else {
            Some(NodeHandle(self.first))
        }
    }


    pub fn last_node_handle(&self) -> Option<NodeHandle<T>> {
        if self.last.is_null() {
            None
        } else {
            Some(NodeHandle(self.last))
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


    /// Get the first node of the bush's top layer
    pub fn first_node_mut(&self) -> Option<&mut BushNode<T>> {
        if self.first.is_null() {
            None
        } else {
            unsafe {
                Some(&mut *self.first)
            }
        }
    }


    /// Get the nth node of the bush's top layer
    pub fn nth_node(&self, i: usize) -> Option<&BushNode<T>> {
        self.iter_nodes().nth(i)
    }


    /// Get the nth item of the bush's top layer
    pub fn nth_item(&self, i: usize) -> Option<&T> {
        self.nth_node(i).map(|node| &node.item)
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


    /// Prepend a new node to the bush's top layer
    pub fn prepend(&mut self, item: T) {

        let node = Box::leak(Box::new(BushNode::new(item, null_mut())));

        if self.first.is_null() {
            // Bush is empty
            self.first = node;
            self.last = node;
        } else {
            // Bush is not empty
            unsafe {
                (*self.first).left = node;
            }
            node.right = self.first;
            self.first = node;
        }
    }


    /// Get an iterator over the items of the bush's top layer
    pub fn iter_items(&self) -> IterItems<T, BushNodeIterRight<T>> {
        self.iter_nodes().map(|node| &node.item)
    }


    /// Get an iterator over the items of the bush's top layer
    pub fn iter_items_mut(&mut self) -> IterItemsMut<T, BushNodeIterRightMut<T>> {
        self.iter_nodes_mut().map(|node| &mut node.item)
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


    /// Get an iterator over the nodes of the bush's top layer
    pub fn iter_nodes_mut(&self) -> BushNodeIterRightMut<'_, T> {
        BushNodeIterRightMut {
            node: if self.first.is_null() {
                None
            } else {
                Some(unsafe { &mut *self.first })
            }
        }
    }


    /// Extrat the given node and its branches, assuming that the node is in the bush's top layer
    /// Assumes the node pointer is not null
    pub fn extract_node(&mut self, node: NodeHandle<T>) -> Box<BushNode<T>> {
        let node_ptr = node.as_ptr() as *mut BushNode<T>;
        let node = node.as_ref();

        if node_ptr != self.first {
            let left_node = node.left;
            // Assume the left node is not null since the current node is not the first
            unsafe {
                (*left_node).right = node.right
            }
        } else {
            self.first = node.right;
        }

        if node_ptr != self.last {
            let right_node = node.right;
            // Assume the right node is not null since the current node is not the last
            unsafe {
                (*right_node).left = node.left;
            }
        } else {
            self.last = node.left;
        }

        unsafe { Box::from_raw(node_ptr) }
    }


    /// Extract a slice of the bush and the relative branches into a new bush, assumimg the nodes are n the bush's top layer
    pub fn extract_slice(&mut self, start_node: NodeHandle<T>, end_node: NodeHandle<T>) -> Bush<T> {
        let start_ptr = start_node.as_ptr() as *mut BushNode<T>;
        let end_ptr = end_node.as_ptr() as *mut BushNode<T>;
        let start_node = start_node.as_ref();
        let end_node = end_node.as_ref();

        if start_ptr != self.first {
            let left_node = start_node.left;
            // Assume the left node is not null
            unsafe {
                (*left_node).right = end_node.right;
            }
        } else {
            self.first = end_node.right;
        }

        if end_ptr != self.last {
            let right_node = end_node.right;
            // Assume the right node is nt null
            unsafe {
                (*right_node).left = start_node.left;
            }
        } else {
            self.last = start_node.left;
        }

        Bush {
            first: start_ptr,
            last: end_ptr
        }
    }


    /// Recursively flatten the bush into the top layer
    pub fn flatten(&mut self) {

        for node in self.iter_nodes_mut() {
            if let Some(mut children) = node.children.take() {
                children.flatten();
                if let Some(children) = children.extract() {
                    node.insert_slice_right(children);
                }
            }
        }

    }


    /// Return the first and last node of the bush's top layer, consuming the bush
    pub fn extract(mut self) -> Option<BushSlice<T>> {
        if self.first.is_null() {
            None
        } else {
            // Set the first and last nodes to null to avoid dropping them when the bush is dropped
            let first = self.first;
            let last = self.last;
            self.first = null_mut();
            self.last = null_mut();
            unsafe {
                Some((Box::from_raw(first), Box::from_raw(last)))
            }
        }
    }


    /// Get a breadth first search iterator over the bush
    pub fn bfs_nodes(&self) -> BFSIter<T> {
        BFSIter {
            nodes: if let Some(first) = self.first_node() {
                VecDeque::from(vec![first])
            } else {
                VecDeque::new()
            }
        }
    }


    /// Get a depth first search iterator over the bush
    pub fn dfs_nodes(&self) -> DFSIter<T> {
        DFSIter {
            nodes: if let Some(first) = self.first_node() {
                VecDeque::from(vec![first])
            } else {
                VecDeque::new()
            }
        }
    }


    /// Get a breadth first search iterator over the bush
    pub fn bfs_items(&self) -> IterItems<T, BFSIter<T>> {
        self.bfs_nodes().map(|node| &node.item)
    }


    /// Get a depth first search iterator over the bush
    pub fn dfs_items(&self) -> IterItems<T, DFSIter<T>> {
        self.dfs_nodes().map(|node| &node.item)
    }

}


impl<T> Default for Bush<T> {
    fn default() -> Self {
        Self {
            first: null_mut(),
            last: null_mut(),
        }
    }
}


pub struct BFSIter<'a, T> {

    nodes: VecDeque<&'a BushNode<T>>,

}


impl<'a, T> Iterator for BFSIter<'a, T> {
    type Item = &'a BushNode<T>;

    fn next(&mut self) -> Option<Self::Item> {
        
        self.nodes.pop_front().map(
            |node|
        {
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

            node
        })
    }
}


pub struct DFSIter<'a, T> {

    nodes: VecDeque<&'a BushNode<T>>,

}


impl<'a, T> Iterator for DFSIter<'a, T> {
    type Item = &'a BushNode<T>;

    fn next(&mut self) -> Option<Self::Item> {

        self.nodes.pop_front().map(
            |node|
        {
            // Push the same-layer nodes before children nodes to give children priority
            if let Some(right) = node.right_node() {
                self.nodes.push_front(right);
            }

            if let Some(children) = &node.children {
                if let Some(first_node) = children.first_node() {
                    self.nodes.push_front(first_node);
                }
            }

            node
        })
    }
}


impl<T> Drop for Bush<T> {
    fn drop(&mut self) {
        let mut node = self.first;

        while !node.is_null() {
            let owned_node = unsafe { Box::from_raw(node) };
            node = owned_node.right;
        }        
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_empty_bush() {
        let bush: Bush<i32> = Bush::new();
        assert_eq!(bush.top_layer_length(), 0);
        assert_eq!(bush.total_node_count(), 0);
    }


    #[test]
    fn append_to_bush() {
        let mut bush = Bush::new();
        for i in 0..10 {
            bush.append(i);
        }
        assert_eq!(bush.top_layer_length(), 10);
        assert_eq!(bush.total_node_count(), 10);
    }


    #[test]
    fn prepend_to_bush() {
        let mut bush = Bush::new();
        for i in 0..10 {
            bush.prepend(i);
        }
        assert_eq!(bush.top_layer_length(), 10);
        assert_eq!(bush.total_node_count(), 10);

        for i in 0..10 {
            assert_eq!(bush.nth_item(i), Some(&(9 - i)));
        }
    }


    #[test]
    fn iter_items() {
        let mut bush = Bush::new();
        for i in 0..10 {
            bush.append(i);
        }
        for (i, item) in bush.iter_items().enumerate() {
            assert_eq!(i, *item as usize);
        }
    }


    #[test]
    fn iter_bush_nodes() {
        let mut bush = Bush::new();
        bush.append(1);
        bush.append(2);
        bush.append(3);
        let mut iter = bush.iter_nodes();
        assert_eq!(iter.next().map(|n| n.item), Some(1));
        assert_eq!(iter.next().map(|n| n.item), Some(2));
        assert_eq!(iter.next().map(|n| n.item), Some(3));
        assert_eq!(iter.next().map(|n| n.item), None);
    }


    #[test]
    fn iter_bush_nodes_mut() {
        let mut bush = Bush::new();
        bush.append(1);
        bush.append(2);
        bush.append(3);
        let mut iter = bush.iter_nodes_mut();
        assert_eq!(iter.next().map(|n| n.item), Some(1));
        assert_eq!(iter.next().map(|n| n.item), Some(2));
        assert_eq!(iter.next().map(|n| n.item), Some(3));
        assert_eq!(iter.next().map(|n| n.item), None);
    }


    #[test]
    fn extract_node() {
        let mut bush = Bush::new();
        bush.append(1);
        bush.append(2);
        bush.append(3);
        let node = bush.extract_node(bush.first_node_handle().unwrap());
        assert_eq!(node.item, 1);
        assert_eq!(bush.top_layer_length(), 2);
        assert_eq!(bush.total_node_count(), 2);
    }

    
    #[test]
    fn extract_slice() {
        let mut bush = Bush::new();
        bush.append(1);
        bush.append(2);
        bush.append(3);
        let slice = bush.extract_slice(bush.first_node_handle().unwrap(), bush.last_node_handle().unwrap());
        assert_eq!(slice.top_layer_length(), 3);
        assert_eq!(bush.top_layer_length(), 0);
        assert_eq!(slice.total_node_count(), 3);
        assert_eq!(bush.total_node_count(), 0);
    }


    #[test]
    fn build_up() {
        let mut bush: Bush<usize> = Bush::new();
        
        let mut counter = 0;

        for _ in 0..10 {
            bush.append(counter);
            counter += 1;

            let mut children = Bush::new();
            for _ in 0..10 {
                children.append(counter);
                counter += 1;
            }
            
            bush.last_node_mut().unwrap().children = Some(children);
        }

        assert_eq!(bush.total_node_count(), counter);
    }


    #[test]
    fn flatten() {
        let mut bush: Bush<usize> = Bush::new();
        
        let mut counter = 0;

        for _ in 0..10 {
            bush.append(counter);
            counter += 1;

            let mut children = Bush::new();
            for _ in 0..10 {
                children.append(counter);
                counter += 1;
            }
            
            bush.last_node_mut().unwrap().children = Some(children);
        }

        assert_eq!(bush.total_node_count(), counter);

        bush.flatten();

        assert_eq!(bush.total_node_count(), counter);

        bush.iter_nodes().enumerate().for_each(|(i, item)| {
            assert_eq!(i, item.item);
        });
    }


    #[test]
    fn extract_bush() {
        let mut bush: Bush<usize> = Bush::new();

        let mut counter = 0;
        for _ in 0..10 {
            bush.append(counter);
            counter += 1;

            let mut children = Bush::new();
            for _ in 0..10 {
                children.append(counter);
                counter += 1;
            }

            bush.last_node_mut().unwrap().children = Some(children);
        }

        let _extracted = bush.extract().unwrap();

    }

    

}

