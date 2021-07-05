-- This file should undo anything in `up.sql`
drop
constraint tokens_pk;
drop
index tokens_token_uindex;
drop table tokens;