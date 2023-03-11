use std::sync::Arc;

use crate::infra::{config, Register, Resolver};

use super::store::{OAuthStoreProxy, BitbucketOAuthStore, GithubOAuthStore, GitlabOAuthStore, AsOAuthStore};

#[derive(Clone)]
pub struct AuthResolver {
    oauth_store: Register<Arc<OAuthStoreProxy>>
}

impl AuthResolver {
    pub fn new(
        github_oauth_config: config::GithubOAuth,
        // gitlab_oauth_config: config::GitlabOAuth,
        // bitbucket_oauth_config: config::BitbucketOAuth,
    ) -> Self {
        let github_oauth_store = GithubOAuthStore::new(github_oauth_config);
        let gitlab_oauth_store = GitlabOAuthStore::new();
        let bitbucket_oauth_store = BitbucketOAuthStore::new();

        let oauth_store = Register::once(Arc::new(OAuthStoreProxy::new(
            github_oauth_store,
            gitlab_oauth_store,
            bitbucket_oauth_store
        )));

        AuthResolver { oauth_store }
    }
}

impl Resolver {
    pub(in crate::modules) fn oauth_proxy(&self) -> impl AsOAuthStore {
        self.resolve(&self.auth_resolver.oauth_store)
    }
}
