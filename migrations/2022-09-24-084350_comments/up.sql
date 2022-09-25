CREATE TABLE "user_comment" (
  "RestaurantID" uuid,
  "User" varchar(30),
  "Comment" text NOT NULL,
  PRIMARY KEY ("RestaurantID", "User")
);

ALTER TABLE "user_comment" ADD FOREIGN KEY ("RestaurantID") REFERENCES "restaurant_info" ("ID");
ALTER TABLE "user_comment" ADD FOREIGN KEY ("User") REFERENCES "user_info" ("Login");
