use std::mem;
use std::ops::ControlFlow::Break;

#[derive(Debug, Clone)]
pub struct Node<T> {
    pub(crate) elem: T,
    pub(crate) pointer: Option<Box<Node<T>>>
}

impl<T> Node<T> {
    pub(crate) fn new(elem: T) -> Node<T>{
        Self{
            elem,
            pointer: None
        }
    }
}

#[derive(Debug)]
pub struct Stack<T> {
    head: Option<Node<T>>,
    largo: i32,
}

impl<T> Stack<T> {
    pub fn new() -> Self{
        Self{
            head: None,
            largo: 0
        }
    }

    pub fn push(&mut self, elem: T){
        let mut node = Node::new(elem);
        if let Some(current_head) = mem::replace(&mut self.head, None){
            node.pointer = Some(Box::new(current_head));
        }
        self.head = Some(node);
        self.largo += 1;
    }

    pub fn peek(&self) -> Option<&T> {
        match &self.head {
            Some(current_head) => Some(&current_head.elem),
            None => None
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        if let Some(current_head) = mem::replace(&mut self.head, None) {
            self.head = match current_head.pointer {
                Some(current_pointer) => Some(*current_pointer),
                None => None,
            };
            self.largo -= 1;
            Some(current_head.elem)
        } else { None }
    }

    pub fn invertir_frase(frase: &str) -> String {
        if frase.len() != 0 {
            let mut pila_char = Stack::new();
            let mut resultado = String::new();
            for char in frase.chars() {
                pila_char.push(char);
            }
            loop {
                let char_from_pila = pila_char.pop().expect("La pila se vacio antes de lo esperado.");
                resultado.push(char_from_pila);
                //pop() reemplaza la cabeza con None si el siguiente estaba vacio, por lo tanto:
                if pila_char.head.is_none() { break; }
            }
            return resultado;
        } else { panic!("Por favor ingresar un string de largo mayor a 0.") }
    }

    pub fn cuenta(pila: &Stack<T>) -> i32 {
        pila.largo
    }
}
