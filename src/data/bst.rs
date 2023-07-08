use std::cell::{Ref, RefCell, RefMut};
use std::rc::Rc;
use std::cmp::{Ord, Ordering};
use std::mem;
use std::sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard};

#[derive(Debug, Clone)]
pub struct NodeTree<T> {
    key: u64,
    data: Arc<T>,
    left: Option<Arc<RwLock<NodeTree<T>>>>,
    right: Option<Arc<RwLock<NodeTree<T>>>>
}

impl<T> NodeTree<T> {
    pub fn new(value: u64, data: T) -> Self {
        Self{
            key: value,
            data: Arc::new(data),
            left: None,
            right: None
        }
    }
}

#[derive(Debug, Clone)]
pub struct BST<T> {
    root: Option<Arc<RwLock<NodeTree<T>>>>
}

impl<T> BST<T> {
    pub fn new() -> Self {
        Self{
            root: None
        }
    }

    pub fn insert(&mut self, key: u64, data: T) {
        let node = Arc::new(RwLock::new(NodeTree::new(key, data)));
        if let Some(root) = &self.root {
            let root_mut_borrow = root.write().unwrap();
            Self::impl_insert(root_mut_borrow, &node);
        } else {
            let clone_node = node.clone();
            self.root = Some(clone_node);
        }
    }

    fn impl_insert(mut current_node: RwLockWriteGuard<NodeTree<T>>, new_node: &Arc<RwLock<NodeTree<T>>>) {
        match new_node.read().unwrap().key.cmp(&current_node.key) {
            //left
            Ordering::Less => {
                if let Some(left) = &current_node.left {
                    Self::impl_insert(left.write().unwrap(), &new_node);
                } else {
                    let clone_node = new_node.clone();
                    current_node.left = Some(clone_node);
                }
            }
            //right
            Ordering::Greater => {
                if let Some(right) = &current_node.right {
                    Self::impl_insert(right.write().unwrap(), &new_node);
                } else {
                    let clone_node = new_node.clone();
                    current_node.right = Some(clone_node);
                }
            }
            //already existing
            Ordering::Equal => return,
        }
    }

    pub fn show(&self) {
        if let Some(root) = &self.root {
            let root_mut_borrow = root.read().unwrap();
            Self::impl_show(root_mut_borrow, "".to_string());
        } else {
            println!("The current BST is empty");
        }
    }

    ///Shows BST as Preorden
    fn impl_show(current_root: RwLockReadGuard<NodeTree<T>>, prefijo: String){
        println!("{}{:?}", prefijo, current_root.key);
        if let Some(left) = &current_root.left {
            Self::impl_show(left.read().unwrap(), format!("{}{}", prefijo, "-   "));
        }
        if let Some(right) = &current_root.right {
            Self::impl_show(right.read().unwrap(),format!("{}{}", prefijo, "-   ") )
        }
    }

    ///replaces the corresponding key data with the given value.
    pub fn replace_by_key<'b>(&mut self, key: u64, value: T) -> Option<()> {
        if let Some(root) = &self.root {
            let root_mut_ref = Arc::clone(root);
            let result_search = Self::impl_get_by_key(root_mut_ref, key);
            if let Some(node) = result_search {
                node.write().unwrap().data = Arc::new(value);
                return Some(());
            }
        } else { return None}
        None
    }

    ///returns a reference to the value corresponding to the key.
    pub fn get_by_key<'b>(&mut self, key: u64) -> Option<Arc<T>> {
        if let Some(root) = &self.root {
            let root_mut_ref = Arc::clone(root);
            let result_search = Self::impl_get_by_key(root_mut_ref, key);
            if let Some(node) = result_search {
                return Some(Arc::clone(&node.read().unwrap().data));
            }
        } else { return None}
        None
    }

    fn impl_get_by_key(current_root: Arc<RwLock<NodeTree<T>>>, key: u64) -> Option<Arc<RwLock<NodeTree<T>>>> {
        match key.cmp(&current_root.read().unwrap().key) {
            Ordering::Less => {
                if let Some(left) = &current_root.read().unwrap().left {
                    if let Some(node) = Self::impl_get_by_key(Arc::clone(left), key){
                        return Some(node);
                    }
                }
            }
            Ordering::Greater => {
                if let Some(right) = &current_root.read().unwrap().right {
                    if let Some(node) = Self::impl_get_by_key(Arc::clone(right), key){
                        return Some(node);
                    }
                }
            }
            Ordering::Equal => {
                return Some(Arc::clone(&current_root));
            }
        }
        None
    }

    pub fn clear(&mut self){
        self.root = None
    }

    pub fn delete_by_key(&mut self, key: u64){
        if let Some(root) = &self.root {
            if root.read().unwrap().key == key {
                Self::delete_node(Arc::clone(root), Arc::clone(&root));
            } else {
                let found = Self::find_and_delete(Arc::clone(&root), key);
                if let Some((node_found, parent)) = found {
                    Self::delete_node(node_found, parent);
                }
            }
        }
    }

    fn find_and_delete(node: Arc<RwLock<NodeTree<T>>>, key: u64) -> Option<(Arc<RwLock<NodeTree<T>>>, Arc<RwLock<NodeTree<T>>>)> {
        match key.cmp(&node.read().unwrap().key) {
            Ordering::Equal => {},
            Ordering::Less => {
                if let Some(left) = &node.read().unwrap().left {
                    return Self::impl_find_and_delete(Arc::clone(left), key, Arc::clone(&node));
                }
            },
            Ordering::Greater => {
                if let Some(right) = &node.read().unwrap().right {
                    return Self::impl_find_and_delete(Arc::clone(right), key, Arc::clone(&node));
                }
            }
        }
        None
    }

    fn impl_find_and_delete(
        current_node: Arc<RwLock<NodeTree<T>>>,
        key: u64,
        parent: Arc<RwLock<NodeTree<T>>>
    ) -> Option<(Arc<RwLock<NodeTree<T>>>, Arc<RwLock<NodeTree<T>>>)>{
        match key.cmp(&current_node.read().unwrap().key) {
            Ordering::Less => {
                if let Some(left) = &current_node.read().unwrap().left {
                    if let Some(found) = Self::impl_find_and_delete(Arc::clone(left), key, Arc::clone(&current_node)){
                        return Some(found);
                    }
                }
            },
            Ordering::Greater => {
                if let Some(right) = &current_node.read().unwrap().right {
                    if let Some(found) = Self::impl_find_and_delete(Arc::clone(right), key, Arc::clone(&current_node)){
                        return Some(found);
                    }
                }
            },
            Ordering::Equal => {
                return Some((Arc::clone(&current_node), Arc::clone(&parent)));
            },
        };
        None
    }

    fn delete_node(current_node: Arc<RwLock<NodeTree<T>>>, parent: Arc<RwLock<NodeTree<T>>>) {
        let left_exist = current_node.read().unwrap().left.is_some();
        let right_exist = current_node.read().unwrap().right.is_some();
        let mut current_node_is_right_child = false;
        if let Some(right) = &parent.read().unwrap().right {
            if right.read().unwrap().key == current_node.read().unwrap().key { current_node_is_right_child = true }
        } else if let Some(left) = &parent.read().unwrap().left {
            if left.read().unwrap().key == current_node.read().unwrap().key { current_node_is_right_child = false }
        }

        //2 childs
        // when deleting we arbitrarily decided to replace it with the node
        // corresponding to the lowest value of the RIGHT subtree.
        if left_exist && right_exist {
            let (smallest_node, parent) = Self::get_min(
                current_node.read().unwrap().right.as_ref().unwrap(),
                &current_node
            );
            let new_key = smallest_node.read().unwrap().key;
            let new_data = Arc::clone(&smallest_node.read().unwrap().data);
            current_node.write().unwrap().key = new_key;
            current_node.write().unwrap().data = new_data;
            Self::delete_node(smallest_node, parent);

            //1 child
        } else if left_exist || right_exist {
            if current_node_is_right_child {
                if let Some(right) = &current_node.read().unwrap().right {
                    let clone_right = Arc::clone(right);
                    let _old_right = mem::replace(&mut parent.write().unwrap().right, Some(clone_right));
                }
                if let Some(left) = &current_node.read().unwrap().left {
                    let clone_left = Arc::clone(left);
                    let _old_left = mem::replace(&mut parent.write().unwrap().right, Some(clone_left));
                }
            } else {
                if let Some(right) = &current_node.read().unwrap().right {
                    let clone_right = Arc::clone(right);
                    let _old_right = mem::replace(&mut parent.write().unwrap().left, Some(clone_right));
                }
                if let Some(left) = &current_node.read().unwrap().left {
                    let clone_left = Arc::clone(left);
                    let _old_left = mem::replace(&mut parent.write().unwrap().left, Some(clone_left));
                }
            }

            //no childs
        } else if !left_exist && !right_exist {
            if current_node_is_right_child {
                {
                    parent.write().unwrap().right = None;
                }
            } else { parent.write().unwrap().left = None }
        }
    }

    ///Gets the node and the parent node corresponding to the lowest key value in the current subtree.
    fn get_min(
        current_node: &Arc<RwLock<NodeTree<T>>>,
        parent: &Arc<RwLock<NodeTree<T>>>
    ) -> (Arc<RwLock<NodeTree<T>>>, Arc<RwLock<NodeTree<T>>>) {
        if let Some(left) = &current_node.read().unwrap().left {
            Self::get_min(left,current_node)
        } else {
            (Arc::clone(current_node), Arc::clone(parent))
        }
    }
}

