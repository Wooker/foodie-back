CREATE TYPE category AS ENUM (
  'FastFood',
  'Ramen',
  'Italian',
  'Japanese'
);

CREATE TABLE "restaurant_category" (
  "restaurant_info_id" uuid,
  "category_type" category,
  "image_url" text NOT NULL,
  PRIMARY KEY ("restaurant_info_id", "category_type")
);

ALTER TABLE "restaurant_category" ADD FOREIGN KEY ("restaurant_info_id") REFERENCES "restaurant_info" ("id");

INSERT INTO restaurant_category VALUES
('85d436f9-3500-4a4b-abea-840fd5e044ec', 'Ramen', 'http://1385980-ci11141.tw1.ru:8080/images/ramen_category.jpg'),
('85d436f9-3500-4a4b-abea-840fd5e044ec', 'Japanese', 'http://1385980-ci11141.tw1.ru:8080/images/japanese_category.jpg'),
('344ce739-adb5-41da-8bd2-189db10cad39', 'FastFood', 'http://1385980-ci11141.tw1.ru:8080/images/fastfood_category.jpg'),
('4d237a1c-0fd0-4f4a-b395-04b4468ecb14', 'Italian', 'http://1385980-ci11141.tw1.ru:8080/images/italian_category.jpg'),
('4d237a1c-0fd0-4f4a-b395-04b4468ecb14', 'FastFood', 'http://1385980-ci11141.tw1.ru:8080/images/fastfood_category.jpg')
