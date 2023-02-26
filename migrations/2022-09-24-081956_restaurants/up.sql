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

INSERT INTO restaurant_info VALUES
('85d436f9-3500-4a4b-abea-840fd5e044ec', 'Name1', 'Description1', 'Address1', 'OpenHours1', 1000, ''),
('344ce739-adb5-41da-8bd2-189db10cad39', 'Name2', 'Description2', 'Address2', 'OpenHours2', 2000, ''),
('4d237a1c-0fd0-4f4a-b395-04b4468ecb14', 'Name3', 'Description3', 'Address3', 'OpenHours3', 3000, '')
;
