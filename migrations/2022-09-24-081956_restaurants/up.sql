CREATE SCHEMA "restaurant";

CREATE TABLE "restaurant_info" (
  "ID" uuid PRIMARY KEY,
  "Name" varchar(20) NOT NULL,
  "Description" text NOT NULL,
  "Address" varchar(30) NOT NULL,
  "OpenHours" varchar(10) NOT NULL,
  "AveragePrice" decimal,
  "Images" text
);

CREATE TABLE "restaurant_location" (
  "RestaurantID" uuid PRIMARY KEY,
  "longitude" float,
  "latitude" float
);

ALTER TABLE "restaurant_location" ADD FOREIGN KEY ("RestaurantID") REFERENCES "restaurant_info" ("ID");
