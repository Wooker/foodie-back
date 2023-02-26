CREATE TABLE "restaurant_menu" (
  "RestaurantID" uuid PRIMARY KEY,
  "FoodName" varchar(20),
  "Price" decimal,
  "Description" text
);

ALTER TABLE "restaurant_menu" ADD FOREIGN KEY ("RestaurantID") REFERENCES "restaurant_info" ("ID");
