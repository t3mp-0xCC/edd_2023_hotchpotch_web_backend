-- Your SQL goes here

CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE "users" (
  "id" Uuid PRIMARY KEY DEFAULT (uuid_generate_v4()),
  "name" varchar(100) NOT NULL,
  "icon_url" varchar(400),
  "created_at" timestamp NOT NULL DEFAULT (now()),
  "updated_at" timestamp NOT NULL DEFAULT (now())
);

CREATE TABLE "events" (
  "id" Uuid PRIMARY KEY DEFAULT (uuid_generate_v4()),
  "name" varchar(100) NOT NULL,
  "desc" varchar(100),
  "url" varchar(400),
  "created_at" timestamp NOT NULL DEFAULT (now()),
  "updated_at" timestamp NOT NULL DEFAULT (now())
);

CREATE TABLE "teams" (
  "id" Uuid PRIMARY KEY DEFAULT (uuid_generate_v4()),
  "event_id" Uuid NOT NULL,
  "reader_id" Uuid NOT NULL,
  "name" varchar(100) NOT NULL,
  "created_at" timestamp NOT NULL DEFAULT (now()),
  "updated_at" timestamp NOT NULL DEFAULT (now())
);

CREATE TABLE "joins" (
  "team_id" Uuid,
  "user_id" Uuid,
  "created_at" timestamp NOT NULL DEFAULT (now()),
  "updated_at" timestamp NOT NULL DEFAULT (now()),
  PRIMARY KEY ("team_id", "user_id")
);

CREATE TABLE "requests" (
  "team_id" Uuid,
  "user_id" Uuid,
  "created_at" timestamp NOT NULL DEFAULT (now()),
  "updated_at" timestamp NOT NULL DEFAULT (now()),
  PRIMARY KEY ("team_id", "user_id")
);

ALTER TABLE "teams" ADD FOREIGN KEY ("event_id") REFERENCES "events" ("id");

ALTER TABLE "teams" ADD FOREIGN KEY ("reader_id") REFERENCES "users" ("id");

ALTER TABLE "joins" ADD FOREIGN KEY ("team_id") REFERENCES "teams" ("id");

ALTER TABLE "joins" ADD FOREIGN KEY ("user_id") REFERENCES "users" ("id");

ALTER TABLE "requests" ADD FOREIGN KEY ("team_id") REFERENCES "teams" ("id");

ALTER TABLE "requests" ADD FOREIGN KEY ("user_id") REFERENCES "users" ("id");

