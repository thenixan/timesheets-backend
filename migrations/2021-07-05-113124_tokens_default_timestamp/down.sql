-- This file should undo anything in `up.sql`
alter table tokens alter column created_at set default null;
alter table tokens alter column last_used_at set default null;