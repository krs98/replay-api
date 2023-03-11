mod bitbucket;
mod github;
mod gitlab;
mod proxy;
mod traits;

pub use self::{
    bitbucket::*,
    github::*,
    gitlab::*,
    proxy::*,
    traits::*
};

