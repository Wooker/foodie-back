CREATE TYPE "tablestatus" AS ENUM (
  'Free',
  'Occupied'
);

CREATE TABLE "restaurant_tables" (
  "id" uuid PRIMARY KEY,
  "restaurant_info_id" uuid NOT NULL,
  "seats" int2 NOT NULL,
  "status" tablestatus DEFAULT 'Free' NOT NULL
);
ALTER TABLE "restaurant_tables" ADD FOREIGN KEY ("restaurant_info_id") REFERENCES "restaurant_info" ("id");

INSERT INTO restaurant_tables VALUES
(gen_random_uuid(), '85d436f9-3500-4a4b-abea-840fd5e044ec', 2),
(gen_random_uuid(), '85d436f9-3500-4a4b-abea-840fd5e044ec', 2),
(gen_random_uuid(), '85d436f9-3500-4a4b-abea-840fd5e044ec', 4),
(gen_random_uuid(), '344ce739-adb5-41da-8bd2-189db10cad39', 4),
(gen_random_uuid(), '4d237a1c-0fd0-4f4a-b395-04b4468ecb14', 4),
(gen_random_uuid(), '006f41e5-ab90-4e86-9e74-c381e8220919', 4);

INSERT INTO restaurant_tables VALUES
(gen_random_uuid(), '85d436f9-3500-4a4b-abea-840fd5e044ec', 2, 'Occupied'),
(gen_random_uuid(), '4d237a1c-0fd0-4f4a-b395-04b4468ecb14', 2, 'Occupied'),
(gen_random_uuid(), '344ce739-adb5-41da-8bd2-189db10cad39', 2, 'Occupied'),
(gen_random_uuid(), '006f41e5-ab90-4e86-9e74-c381e8220919', 2, 'Occupied');
