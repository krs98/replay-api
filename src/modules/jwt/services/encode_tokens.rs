use crate::{modules::jwt::{AccessTokenSubject, RefreshTokenSubject, JwtAccessToken, JwtRefreshToken}, infra::{ServiceArgs, config, Service, Resolver}, Error};

pub struct EncodeTokens {
    pub access_token_subject: AccessTokenSubject,
    pub refresh_token_subject: RefreshTokenSubject
}

impl ServiceArgs for EncodeTokens {
    type Output = Result<(JwtAccessToken, JwtRefreshToken), Error>;
}

async fn execute(
    EncodeTokens { access_token_subject, refresh_token_subject }: EncodeTokens,
    jwt_config: config::Jwt
) -> Result<(JwtAccessToken, JwtRefreshToken), Error> {
    let access_token = {
        let duration = jwt_config.access_token_duration;
        let signature = jwt_config.access_token_secret;

        JwtAccessToken::encode(access_token_subject, duration, signature)
    }?;

    let refresh_token = {
        let duration = jwt_config.refresh_token_duration;
        let signature = jwt_config.refresh_token_secret;

        JwtRefreshToken::encode(refresh_token_subject, duration, signature)
    }?;

    Ok((access_token, refresh_token))
}

impl Resolver {
    pub fn encode_tokens_service(&self) -> impl Service<EncodeTokens> {
        self.service(|resolver, service: EncodeTokens| async move {
            let jwt_config = resolver.jwt_config();
            execute(service, jwt_config).await
        })
    }
}
