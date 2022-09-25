CREATE TABLE "restaurant_rating" (
  "RestaurantID" uuid,
  "User" varchar(30),
  "Rating" int2 NOT NULL,
  PRIMARY KEY ("RestaurantID", "User")
);

ALTER TABLE "restaurant_rating" ADD FOREIGN KEY ("RestaurantID") REFERENCES "restaurant_info" ("ID");
ALTER TABLE "restaurant_rating" ADD FOREIGN KEY ("User") REFERENCES "user_info" ("Login");
