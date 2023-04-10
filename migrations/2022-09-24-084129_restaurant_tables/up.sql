CREATE TYPE "tablestatus" AS ENUM (
  'Free',
  'Occupied'
);

CREATE TABLE "restaurant_tables" (
  "restaurant_info_id" uuid,
  "table_id" int2 NOT NULL,
  "seats" int2 NOT NULL,
  "status" tablestatus DEFAULT 'Free' NOT NULL,
  PRIMARY KEY ("restaurant_info_id", "table_id"),
  UNIQUE ("restaurant_info_id", "table_id")
);
ALTER TABLE "restaurant_tables" ADD FOREIGN KEY ("restaurant_info_id") REFERENCES "restaurant_info" ("id");

INSERT INTO restaurant_tables VALUES
('85d436f9-3500-4a4b-abea-840fd5e044ec', 1, 2),
('85d436f9-3500-4a4b-abea-840fd5e044ec', 2, 2),
('85d436f9-3500-4a4b-abea-840fd5e044ec', 3, 4),
('344ce739-adb5-41da-8bd2-189db10cad39', 1, 4),
('4d237a1c-0fd0-4f4a-b395-04b4468ecb14', 1, 4),
('006f41e5-ab90-4e86-9e74-c381e8220919', 1, 4);

INSERT INTO restaurant_tables VALUES
('85d436f9-3500-4a4b-abea-840fd5e044ec', 4, 2, 'Occupied'),
('4d237a1c-0fd0-4f4a-b395-04b4468ecb14', 2, 2, 'Occupied'),
('344ce739-adb5-41da-8bd2-189db10cad39', 2, 2, 'Occupied'),
('006f41e5-ab90-4e86-9e74-c381e8220919', 2, 2, 'Occupied');
