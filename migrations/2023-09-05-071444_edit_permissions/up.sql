-- Your SQL goes here
ALTER TABLE permissions
DROP COLUMN has_permission,
ADD COLUMN has_daily BOOLEAN NOT NULL DEFAULT FALSE,
ADD COLUMN has_weekly BOOLEAN NOT NULL DEFAULT FALSE,
ADD COLUMN has_monthly BOOLEAN NOT NULL DEFAULT FALSE;