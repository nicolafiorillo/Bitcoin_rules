-- Your SQL goes here

create table headers (
  id              bytea     primary key,
  version         integer   not null,
  previous_block  bytea     not null,
  merkle_root     bytea     not null,
  timestamp       timestamp not null,
  bits            integer   not null,
  nonce           integer   not null
);
