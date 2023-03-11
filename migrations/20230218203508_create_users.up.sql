create table users (
  id bigint primary key,
  username text not null,
  email text,
  created_at timestamp with time zone not null
);

create type oauth_provider as enum ('github', 'gitlab', 'bitbucket');

create table if not exists login_connections (
  id bigint primary key,
  user_id bigint not null references users on delete cascade,
  provider oauth_provider not null,
  access_token text not null,
  created_at timestamp with time zone not null,
  last_connection timestamp with time zone not null
);
