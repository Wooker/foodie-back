CREATE TYPE foodtype AS ENUM (
  'FastFood',
  'Ramen'
);

CREATE TABLE "restaurant_foodtype" (
  "RestaurantID" uuid,
  "FoodType" foodtype,
  PRIMARY KEY ("RestaurantID", "FoodType")
);

CREATE TABLE "restaurant_menu" (
  "RestaurantID" uuid PRIMARY KEY,
  "FoodName" varchar(20),
  "FoodType" foodtype,
  "Price" decimal,
  "Description" text
);

ALTER TABLE "restaurant_foodtype" ADD FOREIGN KEY ("RestaurantID") REFERENCES "restaurant_info" ("ID");
