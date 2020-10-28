

#![allow(dead_code)]


use std::borrow::Cow;
use std::rc::Rc;
use std::cell::RefCell;



#[derive(Debug)]
pub struct Comment {
    id: i32,
    parent_id: Option<i32>,
    path: String,
    children: Vec<Rc<RefCell<Comment>>>
}

impl Comment {
    
    pub fn new( id: i32, parent_id: Option<i32>, path: String ) -> Comment {
        Comment {
            id,
            parent_id,
            path,
            children: vec![]
        }
    }

    // pub fn id( &self ) -> i32 {
    //     self.id
    // }

    // pub fn parent_id( &self ) -> Option<i32> {
    //     self.parent_id
    // }

    // pub fn path( &self ) -> String {
    //     self.path.to_owned()
    // }

    pub fn add_child( &mut self, child: Rc<RefCell<Comment>> ) {
        self.children.push( child )
    }

    pub fn print( &self, white_spaces: usize ) {
        // println!("Self :: {:?}", self);
        let whitespaces = "    ".repeat( white_spaces );
        let sign = if self.children.is_empty() {
            "-"
        } else {
            "+"
        };
        println!("{}{} {}", whitespaces, sign, self.path );
        for child in &self.children {
            child.as_ref().borrow().print( white_spaces + 1 );
        }
    }

}