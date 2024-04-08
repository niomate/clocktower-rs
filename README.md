# clocktower-rs
Tool to track work time.

## Usage:

Install postgresql, create a new user and database and store the path like this in a file called `.env`

DATABASE_URL=postgres://niomate:passwd@localhost/clocktower

Start postgres: sudo -u postgres psql

Create user: create user niomate with password <passwd>;

Create database: create database clocktower;
