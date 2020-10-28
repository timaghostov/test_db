
#![allow(dead_code)]


use std::borrow::Cow;
use std::fs;
use std::path::Path;

use serde::Deserialize;
use toml::{ self, Value as TomlValue, value::Table as TomlTable };

use super::{
    ConfigurationError,
    PgConf
};



#[derive( Debug, Deserialize )]
pub struct Configuration<'a> {
    postgres: Cow<'a, PgConf<'a>>,
    sql: TomlTable
}

impl<'a> Configuration<'a> {

    pub fn open<P: AsRef<Path>>(path: P) -> Result<Configuration<'a>, ConfigurationError> {
        Self::parse( path )?
            // .get("postgres")
            // .ok_or( ConfigurationError::custom("Cannot find 'postgres' section.") )?
            // .clone()
            .try_into()
            .map_err( ConfigurationError::from )

    }
    
    fn parse<P: AsRef<Path>>(path: P) -> Result<TomlValue, ConfigurationError> {
        let content = fs::read_to_string( path )?;
        let value = content.parse::<toml::Value>()?;
        Ok( value )
    }

    pub fn host( &self ) -> Cow<'a, str> {
        self.postgres.host()
    }

    pub fn port( &self ) -> u16 {
        self.postgres.port()
    }

    pub fn user( &self ) -> Cow<'a, str> {
        self.postgres.user()
    }

    pub fn password( &self ) -> Cow<'a, str> {
        self.postgres.password()
    }

    pub fn connection_params( &self ) -> String {
        format!("host={} port={} user={} password={}", self.host(), self.port(), self.user(), self.password() )
    }

    pub fn get_sql_map( &self ) -> TomlTable {
        self.sql.clone()
    }

}