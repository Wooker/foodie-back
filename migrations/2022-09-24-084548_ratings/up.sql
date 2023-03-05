CREATE TABLE "restaurant_rating" (
  "restaurant_info_id" uuid,
  "user_info_id" varchar(30),
  "rating" int2 NOT NULL,
  PRIMARY KEY ("restaurant_info_id", "user_info_id")
);

ALTER TABLE "restaurant_rating" ADD FOREIGN KEY ("restaurant_info_id") REFERENCES "restaurant_info" ("id");
ALTER TABLE "restaurant_rating" ADD FOREIGN KEY ("user_info_id") REFERENCES "user_info" ("id");
