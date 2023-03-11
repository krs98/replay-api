use crate::modules::auth::OAuthProvider;

use super::{GithubOAuthStore, GitlabOAuthStore, BitbucketOAuthStore, DynOAuthStore, AsOAuthStore};

pub struct OAuthStoreProxy {
    github_oauth_store: DynOAuthStore,
    gitlab_oauth_store: DynOAuthStore,
    bitbucket_oauth_store: DynOAuthStore
}

impl OAuthStoreProxy {
    pub fn new(
        github_oauth_store: GithubOAuthStore,
        gitlab_oauth_store: GitlabOAuthStore,
        bitbucket_oauth_store: BitbucketOAuthStore
    ) -> Self {
        OAuthStoreProxy { 
            github_oauth_store: Box::new(github_oauth_store), 
            gitlab_oauth_store: Box::new(gitlab_oauth_store), 
            bitbucket_oauth_store: Box::new(bitbucket_oauth_store)
        }
    }
}

impl AsOAuthStore for OAuthStoreProxy {
    fn as_oauth_store(&self, provider: &OAuthProvider) -> &DynOAuthStore {
        match provider {
            OAuthProvider::Github => &self.github_oauth_store,
            OAuthProvider::Gitlab => &self.gitlab_oauth_store,
            OAuthProvider::Bitbucket => &self.bitbucket_oauth_store
        }
    }
}
