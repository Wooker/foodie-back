CREATE TABLE "user_favorites" (
  "user_info_id" VARCHAR(30),
  "restaurant_info_id" uuid NOT NULL,
  PRIMARY KEY ("user_info_id")
);

ALTER TABLE "user_favorites" ADD FOREIGN KEY ("restaurant_info_id") REFERENCES "restaurant_info" ("id");

INSERT INTO user_favorites VALUES
('user1', '85d436f9-3500-4a4b-abea-840fd5e044ec'),
('user2', '344ce739-adb5-41da-8bd2-189db10cad39')
;
