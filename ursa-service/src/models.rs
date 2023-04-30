use crate::schema::{credentials, issuers};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Debug)]
pub struct Issuer {
    pub id: i32,
    pub name: String,
    pub correctness: String,
    pub privkey: String,
    pub subproofreqparams: String,
}

#[derive(Insertable)]
#[diesel(table_name = issuers)]
pub struct NewIssuer {
    pub name: String,
    pub correctness: String,
    pub privkey: String,
    pub subproofreqparams: String,
}

#[derive(Queryable, Debug, Serialize, Deserialize)]
pub struct Credential {
    pub id: i32,
    pub contractaddr: String,
    pub issuer: String,
    pub walletaddr: String,
    pub credsig: String,
    pub credvalues: String,
}

#[derive(Insertable)]
#[diesel(table_name = credentials)]
pub struct NewCredential {
    pub contractaddr: String,
    pub issuer: String,
    pub walletaddr: String,
    pub credsig: String,
    pub credvalues: String,
}
