use std::ptr::null_mut;


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

    pub fn new(item: T, left: *mut BushNode<T>) -> BushNode<T> {
        Self {
            left,
            right: null_mut(),
            children: None,
            item
        }
    }


    pub fn left_node(&self) -> Option<&BushNode<T>> {
        if self.left.is_null() {
            None
        } else {
            unsafe {
                Some(&*self.left)
            }
        }
    }


    pub fn right_node(&self) -> Option<&BushNode<T>> {
        if self.right.is_null() {
            None
        } else {
            unsafe {
                Some(&*self.right)
            }
        }
    }


    pub fn left_item(&self) -> Option<&T> {
        if self.left.is_null() {
            None
        } else {
            unsafe {
                Some(&(*self.left).item)
            }
        }
    }


    pub fn right_item(&self) -> Option<&T> {
        if self.right.is_null() {
            None
        } else {
            unsafe {
                Some(&(*self.right).item)
            }
        }
    }


    pub fn iter_item_left(&self) -> BushNodeItemIterLeft<'_, T> {
        BushNodeItemIterLeft { 
            node: Some(self)
        }
    }


    pub fn iter_item_right(&self) -> BushNodeItemIterRight<'_, T> {
        BushNodeItemIterRight { 
            node: Some(self)
        }
    }


    pub fn iter_left(&self) -> BushNodeIterLeft<'_, T> {
        BushNodeIterLeft { 
            node: Some(self)
        }
    }


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

    pub fn new() -> Bush<T> {
        Self::default()
    }


    pub fn last_item(&self) -> Option<&T> {
        if self.last.is_null() {
            None
        } else {
            unsafe {
                Some(&(*self.last).item)
            }
        }
    }


    pub fn last_node(&self) -> Option<&BushNode<T>> {
        if self.last.is_null() {
            None
        } else {
            unsafe {
                Some(&*self.last)
            }
        }
    }


    pub fn first_item(&self) -> Option<&T> {
        if self.first.is_null() {
            None
        } else {
            unsafe {
                Some(&(*self.first).item)
            }
        }
    }


    pub fn first_node(&self) -> Option<&BushNode<T>> {
        if self.first.is_null() {
            None
        } else {
            unsafe {
                Some(&*self.first)
            }
        }
    }


    pub fn nth_node(&self, i: usize) -> Option<&BushNode<T>> {
        self.iter_nodes().nth(i)
    }


    pub fn nth_item(&self, i: usize) -> Option<&T> {
        self.iter_items().nth(i)
    }


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


    pub fn iter_items(&self) -> BushNodeItemIterRight<'_, T> {
        BushNodeItemIterRight {
            node: if self.first.is_null() {
                None
            } else {
                Some(unsafe { &*self.first })
            }
        }
    }


    pub fn iter_nodes(&self) -> BushNodeIterRight<'_, T> {
        BushNodeIterRight {
            node: if self.first.is_null() {
                None
            } else {
                Some(unsafe { &*self.first })
            }
        }
    }


    /// Extrat the given node, assuming that the node is in the bush's roots
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

