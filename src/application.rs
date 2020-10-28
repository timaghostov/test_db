


use std::collections::HashMap;
use std::cell::RefCell;
use std::rc::Rc;

use crate::configuration::{
    CONF_PATH,
    Configuration,
};
use crate::postgresql::DBConnection;
use crate::model::Comment;



#[derive(Debug)]
pub struct App;

impl App {
    
    pub fn run( base: &str ) -> Result< (), Box< dyn std::error::Error > > {
        let conf = Configuration::open( CONF_PATH )?;
        let mut conn = DBConnection::connect( &conf )?;

        let _result = conn.prepare_database()?;
        // println!("prepare_database result :: {:?}", result);
        // println!("-------------------------------------------------------");

        let ( parents, comments ) = conn.find_all( base )?;
        
        Self::print_out( parents, comments );

        Ok( () )
    }

    fn print_out( parents: Vec<i32>, comments: HashMap<i32, Rc<RefCell<Comment>>> ) {
        // println!("parents :: {:?}", parents);
        for parent_id in parents {
            match comments.get( &parent_id ) {
                Some( comment ) => {
                    comment.as_ref().borrow().print( 0 );
                },
                None => (),
            }
        }
    }

}