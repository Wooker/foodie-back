CREATE TYPE "cuisine" AS ENUM (
  'European',
  'Japanese'
);

CREATE TABLE "restaurant_cuisine" (
  "RestaurantID" uuid,
  "CuisineType" cuisine NOT NULL,
  PRIMARY KEY ("RestaurantID", "CuisineType")
);

ALTER TABLE "restaurant_cuisine" ADD FOREIGN KEY ("RestaurantID") REFERENCES "restaurant_info" ("ID");
