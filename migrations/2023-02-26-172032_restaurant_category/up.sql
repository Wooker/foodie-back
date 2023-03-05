CREATE TYPE category AS ENUM (
  'FastFood',
  'Ramen',
  'Italian',
  'Japanese'
);

CREATE TABLE "restaurant_category" (
  "restaurant_info_id" uuid,
  "category_type" category,
  PRIMARY KEY ("restaurant_info_id", "category_type")
);

ALTER TABLE "restaurant_category" ADD FOREIGN KEY ("restaurant_info_id") REFERENCES "restaurant_info" ("id");

INSERT INTO restaurant_category VALUES
('85d436f9-3500-4a4b-abea-840fd5e044ec', 'Ramen'),
('85d436f9-3500-4a4b-abea-840fd5e044ec', 'Japanese'),
('344ce739-adb5-41da-8bd2-189db10cad39', 'FastFood'),
('4d237a1c-0fd0-4f4a-b395-04b4468ecb14', 'Italian'),
('4d237a1c-0fd0-4f4a-b395-04b4468ecb14', 'FastFood')
