CREATE TYPE "tablestatus" AS ENUM (
  'Free',
  'Occupied'
);

CREATE TABLE "restaurant_tables" (
  "RestaurantID" uuid,
  "TableNumber" int2,
  "Seats" int2,
  "Status" tablestatus,
  PRIMARY KEY ("RestaurantID", "TableNumber")
);
