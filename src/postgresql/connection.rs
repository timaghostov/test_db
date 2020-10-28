

#![allow(unused_variables, dead_code, unused_imports)]

use std::borrow::Cow;
use std::cell::RefCell;
use std::rc::Rc;
use std::collections::HashMap;

use postgres::{ Client, NoTls, Error };
use toml::value::Table as TomlTable;

use crate::configuration::Configuration;
use crate::model::Comment;
use super::constant::*;
use super::error::SqlError;


pub struct DBConnection {
    client: Client,
    sql_map: TomlTable
}

impl DBConnection {
    
    pub fn connect( conf: &Configuration ) -> Result< DBConnection, Error > {
        let params = conf.connection_params();
        // println!("params :: {}", params);
        Ok( DBConnection {
            client: Client::connect( &params, NoTls )?,
            sql_map: conf.get_sql_map()
        } )
    }

    fn sql_script( &self, key: &str ) -> Option<String> {
        self.sql_map.get( key )
            .and_then( | v | v.as_str() )
            .map( | s | String::from( s ) )
    }

    pub fn prepare_database( &mut self ) -> Result< (), SqlError > {
        let script_opt = {
            self.sql_script( KEY_PREPARE_DB )
        };
        match script_opt {
            Some( sql_script ) => {
                self.client.batch_execute( &sql_script )?;
                Ok( () )
            },
            None => Err( SqlError::ScriptNotFound ),
        }
    }

    pub fn find_all( &mut self, base: &str ) -> Result< ( Vec<i32>, HashMap<i32, Rc<RefCell<Comment>>> ), SqlError > {
        let script_opt = {
            self.sql_script( KEY_FIND_ALL )
        };
        match script_opt {
            Some( sql_script ) => {                
                // let mut list = vec![];
                let mut map: HashMap<i32, Rc<RefCell<Comment>>> = HashMap::new();
                let mut parents = vec![];

                for row in self.client.query( sql_script.as_str(), &[ &base ] )? {
                    let id = row.get( "id" );
                    let parent_id_opt = row.get( "parent_id" );
                    // let path = Cow::Owned( row.get::<&str, String>( "path" ) );
                    let path = row.get( "path" );

                    let comment = Rc::new( RefCell::new( Comment::new( id, parent_id_opt, path ) ) );

                    if let Some( parent_id ) = parent_id_opt {
                        if id == 0 {
                            parents.push( id );
                        }
                        match map.get( &parent_id ) {
                            Some( parent ) => {
                                (*parent.borrow_mut()).add_child( Rc::clone( &comment ) );
                            },
                            None => (),
                        }
                    }
                    
                    map.insert( id, comment );
                    
                    // list.push( Comment::new( id, parent_id, path ) );
                }
                Ok( ( parents, map ) )
            },
            None => Err( SqlError::ScriptNotFound ),
        }
    }

    

}