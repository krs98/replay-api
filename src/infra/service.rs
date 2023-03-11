use std::future::Future;

use async_trait::async_trait;

use super::Resolver;

pub trait ServiceArgs {
    type Output;
}

#[async_trait]
pub trait Service<Args: ServiceArgs> {
    async fn execute(&self, input: Args) -> Args::Output;
}

#[async_trait]
impl<TArgs, TService, TFuture> Service<TArgs> for TService
where
    TArgs: ServiceArgs + Send + 'static,
    TService: Fn(TArgs) -> TFuture + Send + Sync,
    TFuture: Future<Output = TArgs::Output> + Send,
{
    async fn execute(&self, input: TArgs) -> TArgs::Output {
        self(input).await
    }
}

impl Resolver {
    pub fn service<TArgs, TService, TFuture>(&self, service: TService) -> impl Service<TArgs>
    where
        TArgs: ServiceArgs + Send + 'static,
        TService: Fn(Resolver, TArgs) -> TFuture + Send + Sync,
        TFuture: Future<Output = TArgs::Output> + Send,
    {
        let resolver = self.by_ref();
        move |input: TArgs| {
            let resolver = resolver.by_ref();
            service(resolver, input)
        }
    }
}
