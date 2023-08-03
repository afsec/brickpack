CREATE TABLE IF NOT EXISTS `applets` (
    "id" INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT UNIQUE,
    "oid" TEXT NOT NULL UNIQUE,
    "filename" TEXT NOT NULL,
    "code" TEXT NOT NULL,
    "size" INTEGER NOT NULL,
    "created_at" TIMESTAMP NOT NULL,
    "updated_at" TIMESTAMP NOT NULL,
    "is_deleted" BOOLEAN NOT NULL
);

CREATE TABLE IF NOT EXISTS `departments` (
    "id" INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT UNIQUE,
    "oid" TEXT NOT NULL UNIQUE,
    "name" TEXT NOT NULL UNIQUE,
    "created_at" TIMESTAMP NOT NULL,
    "updated_at" TIMESTAMP NOT NULL,
    "is_deleted" BOOLEAN NOT NULL
);

CREATE TABLE IF NOT EXISTS `permissions` (
    "id" INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT UNIQUE,
    "oid" TEXT NOT NULL UNIQUE,
    "name" TEXT NOT NULL UNIQUE,
    "created_at" TIMESTAMP NOT NULL,
    "updated_at" TIMESTAMP NOT NULL,
    "is_deleted" BOOLEAN NOT NULL
);

CREATE TABLE IF NOT EXISTS `statuses` (
    "id" INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT UNIQUE,
    "oid" TEXT NOT NULL UNIQUE,
    "name" TEXT NOT NULL UNIQUE,
    "created_at" TIMESTAMP NOT NULL,
    "updated_at" TIMESTAMP NOT NULL,
    "is_deleted" BOOLEAN NOT NULL
);

CREATE TABLE IF NOT EXISTS `users` (
    "id" INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT UNIQUE,
    "oid" TEXT NOT NULL UNIQUE,
    "email" TEXT NOT NULL UNIQUE,
    "name" TEXT NOT NULL,
    "department" TEXT NOT NULL,
    "permission" TEXT NOT NULL,
    "status" TEXT NOT NULL,
    "created_at" TIMESTAMP NOT NULL,
    "updated_at" TIMESTAMP NOT NULL,
    "is_deleted" BOOLEAN NOT NULL,
    FOREIGN KEY(department) REFERENCES departments(oid) FOREIGN KEY(permission) REFERENCES permissions(oid) FOREIGN KEY(status) REFERENCES statuses(oid)
);

CREATE UNIQUE INDEX IF NOT EXISTS idx_applets_oid ON applets(oid);

CREATE UNIQUE INDEX IF NOT EXISTS idx_departments_oid ON departments(oid);

CREATE UNIQUE INDEX IF NOT EXISTS idx_permissions_oid ON permissions(oid);

CREATE UNIQUE INDEX IF NOT EXISTS idx_statuses_oid ON statuses(oid);

CREATE UNIQUE INDEX IF NOT EXISTS idx_users_oid ON users(oid);