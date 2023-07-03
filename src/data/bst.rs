use std::cell::{Ref, RefCell, RefMut};
use std::rc::Rc;
use std::cmp::{Ord, Ordering};
use std::mem;

#[derive(Debug, Clone)]
pub struct NodeTree<T> {
    key: u64,
    data: Rc<T>,
    left: Option<Rc<RefCell<NodeTree<T>>>>,
    right: Option<Rc<RefCell<NodeTree<T>>>>
}

impl<T> NodeTree<T> {
    pub fn new(value: u64, data: T) -> Self {
        Self{
            key: value,
            data: Rc::new(data),
            left: None,
            right: None
        }
    }
}

#[derive(Debug, Clone)]
pub struct BST<T> {
    root: Option<Rc<RefCell<NodeTree<T>>>>
}

impl<T: 'static + std::fmt::Display + std::fmt::Debug> BST<T> {
    pub fn new() -> Self {
        Self{
            root: None
        }
    }

    pub fn insert(&mut self, key: u64, data: T) {
        let node = Rc::new(RefCell::new(NodeTree::new(key, data)));
        if let Some(root) = &self.root {
            let root_mut_borrow = root.borrow_mut();
            Self::impl_insert(root_mut_borrow, &node);
        } else {
            let clone_node = node.clone();
            self.root = Some(clone_node);
        }
    }

    fn impl_insert(mut current_node: RefMut<NodeTree<T>>, new_node: &Rc<RefCell<NodeTree<T>>>) {
        match new_node.borrow().key.cmp(&current_node.key) {
            //left
            Ordering::Less => {
                if let Some(left) = &current_node.left {
                    Self::impl_insert(left.borrow_mut(), &new_node);
                } else {
                    let clone_node = new_node.clone();
                    current_node.left = Some(clone_node);
                }
            }
            //right
            Ordering::Greater => {
                if let Some(right) = &current_node.right {
                    Self::impl_insert(right.borrow_mut(), &new_node);
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
            let root_mut_borrow = root.borrow();
            Self::impl_show(root_mut_borrow, "".to_string());
        } else {
            println!("The current BST is empty");
        }
    }

    ///Shows BST as Preorden
    fn impl_show(current_root: Ref<NodeTree<T>>, prefijo: String){
        println!("{}{:?}, data: {:?}", prefijo, current_root.key, current_root.data);
        if let Some(left) = &current_root.left {
            Self::impl_show(left.borrow(), format!("{}{}", prefijo, "-   "));
        }
        if let Some(right) = &current_root.right {
            Self::impl_show(right.borrow(),format!("{}{}", prefijo, "-   ") )
        }
    }

    ///replaces the corresponding key's data with the given value.
    pub fn replace_by_key<'b>(&mut self, key: u64, value: T) -> Option<()> {
        if let Some(root) = &self.root {
            let root_mut_ref = Rc::clone(root);
            let result_search = Self::impl_get_by_key(root_mut_ref, key);
            if let Some(mut node) = result_search {
                //let mut node_ref = node.borrow_mut();
                node.borrow_mut().data = Rc::new(value);
                return Some(());
            }
        } else { return None}
        None
    }



    ///returns a mutable reference to the value corresponding to the key.
    /*pub fn get_by_key<'b>(&mut self, key: u64, value: T) -> Option<&T> {
        if let Some(root) = &self.root {
            let root_mut_ref = Rc::clone(root);
            let var = Self::impl_get_by_key(root_mut_ref, key);
            if let Some(value) = var {

                //let mut borrow = refcell.borrow_mut();
                //let data_borrow = RefMut::map(borrow.data.as_mut().unwrap().borrow_mut(), |value| value);

                //return Some(RefMut::map(data_borrow, |value| value));

                //let mut data = borrow.data.as_mut().unwrap();
                //let mut clone_data = Rc::clone(data);
                //clone_data = Rc::new(RefCell::new(value));
                return Some(value);
                //return Some(Rc::clone(data));
            }
            /*if let Some(value) = var {
                let new_value = value.borrow();
                return Ref::map(new_value, |value| &value.data);
            } else { return None }*/
        } else { return None}
        None
    }*/



    fn impl_get_by_key(current_root: Rc<RefCell<NodeTree<T>>>, key: u64) -> Option<Rc<RefCell<NodeTree<T>>>> {
        println!("current_root: {:?}", current_root.borrow().key);
        match key.cmp(&current_root.borrow().key) {
            Ordering::Less => {
                if let Some(left) = &current_root.borrow().left {
                    if let Some(node) = Self::impl_get_by_key(Rc::clone(left), key){
                        return Some(node);
                    }
                }
            }
            Ordering::Greater => {
                if let Some(right) = &current_root.borrow().right {
                    if let Some(node) = Self::impl_get_by_key(Rc::clone(right), key){
                        return Some(node);
                    }
                }
            }
            Ordering::Equal => {
                println!("got it");
                return Some(Rc::clone(&current_root));
            }
        }
        None
    }


    pub fn clear(&mut self){
        self.root = None
    }

    pub fn delete_by_key(&mut self, key: u64){
        if let Some(root) = &self.root {
            if root.borrow().key == key {
                //eliminar root
            } else {
                let found = Self::find_and_delete(Rc::clone(&root), key);
                if let Some((node_found, parent)) = found {
                    println!("deleting");
                    Self::delete_node(node_found, parent);
                }
            }

        }
    }

    fn find_and_delete(node: Rc<RefCell<NodeTree<T>>>, key: u64) -> Option<(Rc<RefCell<NodeTree<T>>>, Rc<RefCell<NodeTree<T>>>)> {
        match key.cmp(&node.borrow().key) {
            Ordering::Equal => {
                // todo:
                //Self::delete_root();
            },
            Ordering::Less => {
                if let Some(left) = &node.borrow().left {
                    return Self::impl_find_and_delete(Rc::clone(left), key, Rc::clone(&node));
                }
            },
            Ordering::Greater => {
                if let Some(right) = &node.borrow().right {
                    return Self::impl_find_and_delete(Rc::clone(right), key, Rc::clone(&node));
                }
            }
        }
        None
    }

    fn impl_find_and_delete(
        current_node: Rc<RefCell<NodeTree<T>>>,
        key: u64,
        parent: Rc<RefCell<NodeTree<T>>>
    ) -> Option<(Rc<RefCell<NodeTree<T>>>, Rc<RefCell<NodeTree<T>>>)>{
        match key.cmp(&current_node.borrow().key) {
            Ordering::Less => {
                if let Some(left) = &current_node.borrow().left {
                    if let Some(found) = Self::impl_find_and_delete(
                        Rc::clone(left), key,
                        Rc::clone(&current_node)){
                        return Some(found);
                    }
                }
            },
            Ordering::Greater => {
                if let Some(right) = &current_node.borrow().right {
                    if let Some(found) = Self::impl_find_and_delete(
                        Rc::clone(right), key,
                        Rc::clone(&current_node)){
                        return Some(found);
                    }
                }
            },
            Ordering::Equal => {
                println!("found it");
                return Some((Rc::clone(&current_node), Rc::clone(&parent)));
            },
        };
        None
    }

    fn delete_node(current_node: Rc<RefCell<NodeTree<T>>>, parent: Rc<RefCell<NodeTree<T>>>) {
        println!("current node: {:?}", current_node);
        println!("parent node; {:?}", parent);
        let left_exist = current_node.borrow().left.is_some();
        let right_exist = current_node.borrow().right.is_some();
        let mut current_node_is_right_child = false;
        if let Some(right) = &parent.borrow().right {
            if right.borrow().key == current_node.borrow().key { current_node_is_right_child = true }
        } else if let Some(left) = &parent.borrow().left {
            if left.borrow().key == current_node.borrow().key { current_node_is_right_child = false }
        }
        println!("is parent right child: {}", current_node_is_right_child);
        println!("{} {}", left_exist, right_exist);

        //2 childs
        if left_exist && right_exist {
            println!("the choosen {:?}", current_node.borrow().right.as_ref().unwrap());
            let (smallest_node, parent) = Self::get_min(
                current_node.borrow().right.as_ref().unwrap(),
                &current_node
            );
            let new_key = smallest_node.borrow().key;
            let new_data = Rc::clone(&smallest_node.borrow().data);
            current_node.borrow_mut().key = new_key;
            current_node.borrow_mut().data = new_data;
            println!("new current node: {:?}", current_node);
            Self::delete_node(smallest_node, parent);

            //1 child
        } else if left_exist || right_exist {
            println!("1 child, deleting and replacing");
            if current_node_is_right_child {
                if let Some(right) = &current_node.borrow().right {
                    let clone_right = Rc::clone(right);
                    let _old_right = mem::replace(&mut parent.borrow_mut().right, Some(clone_right));
                }
                if let Some(left) = &current_node.borrow().left {
                    let clone_left = Rc::clone(left);
                    let _old_left = mem::replace(&mut parent.borrow_mut().right, Some(clone_left));
                }
            } else {
                if let Some(right) = &current_node.borrow().right {
                    let clone_right = Rc::clone(right);
                    let _old_right = mem::replace(&mut parent.borrow_mut().left, Some(clone_right));
                }
                if let Some(left) = &current_node.borrow().left {
                    let clone_left = Rc::clone(left);
                    let _old_left = mem::replace(&mut parent.borrow_mut().left, Some(clone_left));
                }
            }

            //no childs
        } else if !left_exist && !right_exist {
            println!("no childs, deleting");
            if current_node_is_right_child {
                {
                    parent.borrow_mut().right = None;
                }
            } else { parent.borrow_mut().left = None }
        }
    }

    fn get_min(
        current_node: &Rc<RefCell<NodeTree<T>>>,
        parent: &Rc<RefCell<NodeTree<T>>>
    ) -> (Rc<RefCell<NodeTree<T>>>, Rc<RefCell<NodeTree<T>>>)  {
        if let Some(left) = &current_node.borrow().left {
            Self::get_min(left,current_node)
        } else {
            (Rc::clone(current_node), Rc::clone(parent))
        }
    }
}
