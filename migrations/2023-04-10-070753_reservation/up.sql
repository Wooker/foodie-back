-- Your SQL goes here

CREATE TABLE "reservations" (
  "restaurant_info_id" uuid NOT NULL,
  "table_id" int2 NOT NULL,
  "user_id"  varchar(30) NOT NULL,
  PRIMARY KEY ("restaurant_info_id", "table_id", "user_id")
);
ALTER TABLE "reservations" ADD FOREIGN KEY ("restaurant_info_id", "table_id") REFERENCES "restaurant_tables" ("restaurant_info_id", "table_id");
ALTER TABLE "reservations" ADD FOREIGN KEY ("user_id") REFERENCES "user_info" ("id");
