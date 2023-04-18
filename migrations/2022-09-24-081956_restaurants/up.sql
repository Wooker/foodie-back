CREATE SCHEMA "restaurant";

CREATE TABLE "restaurant_info" (
  "id" uuid PRIMARY KEY,
  "name" varchar(20) NOT NULL,
  "description" text NOT NULL,
  "address" varchar(30) NOT NULL,
  "opening_hours" varchar(11) NOT NULL,
  "price_range" int2 NOT NULL,
  "rating" float4 NOT NULL,
  "image_url" text NOT NULL,
  "contact" text NOT NULL
);

CREATE TABLE "restaurant_location" (
  "restaurant_info_id" uuid,
  "longitude" float4,
  "latitude" float4,
  PRIMARY KEY ("restaurant_info_id", "longitude", "latitude")
);

ALTER TABLE "restaurant_location" ADD FOREIGN KEY ("restaurant_info_id") REFERENCES "restaurant_info" ("id");

INSERT INTO restaurant_info VALUES
('85d436f9-3500-4a4b-abea-840fd5e044ec', 'Zina', 'Zina restaurant - "В гостях у Бабушки Зины"', 'Akyrtas street, 1/1', '08:00-24:00', 4, 4.32, 'http://1385980-ci11141.tw1.ru:8080/images/zina.jpg', '+7(777)123-45-67'),
('344ce739-adb5-41da-8bd2-189db10cad39', 'Qazaq Gourmet', 'Qazaq gourmet restaurant', 'Mangilik el street, 29', '12:00-23:00', 4, 4.53, 'http://1385980-ci11141.tw1.ru:8080/images/qazaq_gourmet.jpg', '+7(777)133-45-67'),
('4d237a1c-0fd0-4f4a-b395-04b4468ecb14', 'Meat point', 'Halal restaurant', 'Uly dala street, 45', '00:00-24:00', 3, 4.19, 'http://1385980-ci11141.tw1.ru:8080/images/meat_point.png', '+7(777)123-55-67'),
('006f41e5-ab90-4e86-9e74-c381e8220919', 'Lanzhou', 'Chinese restaurant', 'Sarayshik street, 5', '11:00-23:00', 3, 4.4, 'http://1385980-ci11141.tw1.ru:8080/images/lanzhou.png', '+7(777)123-45-77')
;

INSERT INTO restaurant_location VALUES
('85d436f9-3500-4a4b-abea-840fd5e044ec', 20.2, 30.3),
('344ce739-adb5-41da-8bd2-189db10cad39', 30.2, 40.3),
('4d237a1c-0fd0-4f4a-b395-04b4468ecb14', 50.2, 60.3),
('006f41e5-ab90-4e86-9e74-c381e8220919', 50.2, 60.3)
;
