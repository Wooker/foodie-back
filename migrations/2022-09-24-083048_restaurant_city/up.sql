CREATE TABLE "city" (
  "ID" uuid PRIMARY KEY NOT NULL,
  "Name" varchar(20) NOT NULL
);

CREATE TABLE "restaurant_city" (
  "RestaurantID" uuid,
  "CityID" uuid NOT NULL,
  PRIMARY KEY ("RestaurantID", "CityID")
);

ALTER TABLE "restaurant_city" ADD FOREIGN KEY ("RestaurantID") REFERENCES "restaurant_info" ("ID");
ALTER TABLE "restaurant_city" ADD FOREIGN KEY ("CityID") REFERENCES "city" ("ID");
