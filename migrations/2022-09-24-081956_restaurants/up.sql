CREATE SCHEMA "restaurant";

CREATE TABLE "restaurant_info" (
  "id" uuid PRIMARY KEY,
  "name" varchar(20) NOT NULL,
  "description" text NOT NULL,
  "address" varchar(30) NOT NULL,
  "openHours" varchar(10) NOT NULL,
  "averagePrice" decimal,
  "images" text
);

CREATE TABLE "restaurant_location" (
  "restaurant_info_id" uuid,
  "longitude" float4,
  "latitude" float4,
  PRIMARY KEY ("restaurant_info_id", "longitude", "latitude")
);

ALTER TABLE "restaurant_location" ADD FOREIGN KEY ("restaurant_info_id") REFERENCES "restaurant_info" ("id");

INSERT INTO restaurant_info VALUES
('85d436f9-3500-4a4b-abea-840fd5e044ec', 'Name1', 'Description1', 'Address1', 'OpenHours1', 1000, ''),
('344ce739-adb5-41da-8bd2-189db10cad39', 'Name2', 'Description2', 'Address2', 'OpenHours2', 2000, ''),
('4d237a1c-0fd0-4f4a-b395-04b4468ecb14', 'Name3', 'Description3', 'Address3', 'OpenHours3', 3000, '')
;

INSERT INTO restaurant_location VALUES
('85d436f9-3500-4a4b-abea-840fd5e044ec', 20.2, 30.3),
('344ce739-adb5-41da-8bd2-189db10cad39', 30.2, 40.3),
('4d237a1c-0fd0-4f4a-b395-04b4468ecb14', 50.2, 60.3)
;
