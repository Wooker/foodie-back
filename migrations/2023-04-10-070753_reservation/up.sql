-- Your SQL goes here

CREATE TABLE "reservations" (
  "id" uuid PRIMARY KEY,
  "table_id" uuid NOT NULL,
  "user_id"  varchar(30) NOT NULL,
  "reservation_start" timestamp NOT NULL,
  "reservation_end" timestamp NOT NULL,
  "personas" int2 NOT NULL
);
ALTER TABLE "reservations" ADD FOREIGN KEY ("table_id") REFERENCES "restaurant_tables" ("id");
ALTER TABLE "reservations" ADD FOREIGN KEY ("user_id") REFERENCES "user_info" ("id");

