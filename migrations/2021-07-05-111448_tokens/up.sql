-- Your SQL goes here
create table tokens
(
    token varchar not null,
    user_id uuid not null
        constraint tokens_to_users
            references users
            on delete cascade,
    created_at timestamp not null,
    last_used_at timestamp not null
);

create unique index tokens_token_uindex
	on tokens (token);

alter table tokens
    add constraint tokens_pk
        primary key (token);

