CREATE TABLE "user_comment" (
  "restaurant_info_id" uuid,
  "user_info_id" varchar(30),
  "comment" text NOT NULL,
  PRIMARY KEY ("restaurant_info_id", "user_info_id")
);

ALTER TABLE "user_comment" ADD FOREIGN KEY ("restaurant_info_id") REFERENCES "restaurant_info" ("id");
ALTER TABLE "user_comment" ADD FOREIGN KEY ("user_info_id") REFERENCES "user_info" ("id");
