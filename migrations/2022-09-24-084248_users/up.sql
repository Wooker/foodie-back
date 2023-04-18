CREATE TABLE "user_info" (
  "id" varchar(30) PRIMARY KEY,
  "password" varchar(20),
  "full_name" varchar(50) NOT NULL,
  "phone_number" varchar(20) NOT NULL,
  "image_url" text NOT NULL
);

INSERT INTO user_info VALUES
('user1', '123', 'Test Test', '+7(701)777-77-77', ''),
('user2', 'password', 'Test2 Test2', '+7(701)777-77-88', ''),
('bunnytest1', 'bunnytest1', 'Bunny', '+7(701)777-77-99', '')
;
