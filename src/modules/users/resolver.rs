use std::sync::Arc;

use sqlx::PgPool;

use crate::infra::{Register, Resolver};

use super::{store::{PgUserStore, self}, UserStore};

#[derive(Clone)]
pub struct UsersResolver {
    user_store: Register<Arc<PgUserStore>>
}

impl UsersResolver {
    pub fn new(pool: PgPool) -> Self {
        UsersResolver { 
            user_store: Register::once(Arc::new(store::PgUserStore::new(pool)))
        }
    }
}

impl Resolver {
    pub(in crate::modules) fn user_store(&self) -> impl UserStore {
        self.resolve(&self.users_resolver.user_store)
    }
}
