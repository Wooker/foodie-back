CREATE TYPE category AS ENUM (
  'FastFood',
  'Ramen',
  'Italian',
  'Japanese'
);

CREATE TABLE "restaurant_category" (
  "RestaurantID" uuid,
  "CategoryType" category,
  PRIMARY KEY ("RestaurantID", "CategoryType")
);

ALTER TABLE "restaurant_category" ADD FOREIGN KEY ("RestaurantID") REFERENCES "restaurant_info" ("ID");
