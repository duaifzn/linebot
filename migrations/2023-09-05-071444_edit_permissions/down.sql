-- This file should undo anything in `up.sql`
ALTER TABLE permissions
ADD COLUMN has_permission BOOLEAN NOT NULL DEFAULT FALSE,
DROP COLUMN has_daily,
DROP COLUMN has_weekly,
DROP COLUMN has_monthly;