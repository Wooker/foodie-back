CREATE TABLE "user_favorites" (
  "Login" VARCHAR(30),
  "RestaurantID" uuid NOT NULL,
  PRIMARY KEY ("Login")
);

ALTER TABLE "user_favorites" ADD FOREIGN KEY ("RestaurantID") REFERENCES "restaurant_info" ("ID");

INSERT INTO user_favorites VALUES
('user1', '85d436f9-3500-4a4b-abea-840fd5e044ec'),
('user2', '344ce739-adb5-41da-8bd2-189db10cad39')
;
