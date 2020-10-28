


use std::fmt::{ self, Display };
use std::error::Error as StdError;
use postgres::Error;


#[derive(Debug)]
pub enum SqlError {
    ScriptNotFound,
    PgError( Error ),
}

impl Display for SqlError {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SqlError::ScriptNotFound => formatter.write_str( "ScriptNotFound" ),
            SqlError::PgError( error ) => formatter.write_str( &error.to_string() ),
        }
    }
}

impl StdError for SqlError {}

impl From< Error > for SqlError {

    fn from( error: Error ) -> SqlError {
        SqlError::PgError( error )
    }

}