CREATE TABLE "restaurant_menu" (
  "id" uuid,
  "restaurant_info_id" uuid,
  "name" varchar(30) NOT NULL,
  "menu_category" category NOT NULL,
  "price" int NOT NULL,
  "ingredients" text NOT NULL,
  "image_url" text NOT NULL,
  PRIMARY KEY("restaurant_info_id", "id")
);

ALTER TABLE "restaurant_menu" ADD FOREIGN KEY ("restaurant_info_id") REFERENCES "restaurant_info" ("id");

INSERT INTO "restaurant_menu" VALUES
(gen_random_uuid(), '006f41e5-ab90-4e86-9e74-c381e8220919', 'Noodles', 'Ramen', 2000, 'Noodles, beef, onion, egg, soup', 'http://1385980-ci11141.tw1.ru:8080/images/Noodles.jpg'),
(gen_random_uuid(), '4d237a1c-0fd0-4f4a-b395-04b4468ecb14', 'Hamburger', 'FastFood', 2600, 'Beef, salad, red onion, pickles, tomato', 'http://1385980-ci11141.tw1.ru:8080/images/burger.png')
;
