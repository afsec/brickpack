#!/bin/sh
DATABASE_URL=sqlite:./target/database.sqlite3 sqlx database create
DATABASE_URL=sqlite:./target/database.sqlite3 sqlx migrate run
DATABASE_URL=sqlite:./target/database.sqlite3 cargo sqlx prepare
