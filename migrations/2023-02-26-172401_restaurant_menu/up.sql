CREATE TABLE "restaurant_menu" (
  "restaurant_info_id" uuid PRIMARY KEY,
  "food_name" varchar(20),
  "price" decimal,
  "description" text
);

ALTER TABLE "restaurant_menu" ADD FOREIGN KEY ("restaurant_info_id") REFERENCES "restaurant_info" ("id");
