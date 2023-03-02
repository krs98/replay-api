create table if not exists users (
  id serial primary key,
  username text not null,
  email text not null,
  password text not null,
  created_at timestamp with time zone not null
);
