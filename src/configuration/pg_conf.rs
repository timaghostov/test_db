


use std::borrow::Cow;
use serde::Deserialize;


#[derive( Debug, Deserialize, Clone )]
pub struct PgConf<'a> {
    host: Cow<'a, str>,
    port: u16,
    user: Cow<'a, str>,
    password: Cow<'a, str>,
}

impl<'a> PgConf<'a> {
    
    pub fn host( &self ) -> Cow<'a, str> {
        self.host.to_owned()
    }

    pub fn port( &self ) -> u16 {
        self.port
    }

    pub fn user( &self ) -> Cow<'a, str> {
        self.user.to_owned()
    }

    pub fn password( &self ) -> Cow<'a, str> {
        self.password.to_owned()
    }

}