use async_trait::async_trait;
use icann_rdap_common::response::{
    autnum::Autnum, domain::Domain, entity::Entity, nameserver::Nameserver, network::Network,
};
use sqlx::{PgPool, Postgres};

use crate::{error::RdapServerError, storage::TxHandle};

pub struct PgTx<'a> {
    db_tx: sqlx::Transaction<'a, Postgres>,
}

impl<'a> PgTx<'a> {
    pub async fn new(pg_pool: &PgPool) -> Result<PgTx<'a>, RdapServerError> {
        let db_tx = pg_pool.begin().await?;
        Ok(PgTx { db_tx })
    }

    pub async fn new_truncate(pg_pool: &PgPool) -> Result<PgTx<'a>, RdapServerError> {
        let mut db_tx = pg_pool.begin().await?;
        // TODO actually complete this
        // this is just here to make sure something will compile
        sqlx::query("truncate domain").execute(&mut db_tx).await?;
        Ok(PgTx { db_tx })
    }
}

#[async_trait]
impl<'a> TxHandle for PgTx<'a> {
    async fn add_entity(&mut self, _entity: &Entity) -> Result<(), RdapServerError> {
        todo!()
    }

    async fn add_domain(&mut self, _domain: &Domain) -> Result<(), RdapServerError> {
        // TODO actually complete this
        // this is just here to make sure something will compile
        sqlx::query("insert domain")
            .execute(&mut self.db_tx)
            .await?;
        Ok(())
    }

    async fn add_nameserver(&mut self, _nameserver: &Nameserver) -> Result<(), RdapServerError> {
        todo!()
    }

    async fn add_autnum(&mut self, _autnum: &Autnum) -> Result<(), RdapServerError> {
        todo!()
    }

    async fn add_network(&mut self, _network: &Network) -> Result<(), RdapServerError> {
        todo!()
    }

    async fn commit(self: Box<Self>) -> Result<(), RdapServerError> {
        self.db_tx.commit().await?;
        Ok(())
    }

    async fn rollback(self: Box<Self>) -> Result<(), RdapServerError> {
        self.db_tx.rollback().await?;
        Ok(())
    }
}