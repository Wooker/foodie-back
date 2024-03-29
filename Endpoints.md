# Endpoints

## Restaurants
```txt
GET /restaurants
```
Data:
```json
[
	{
		"address": "Akyrtas street, 1/1",
		"categories": [
			"Ramen",
			"Japanese"
		],
		"contact": "+7(777)123-45-67",
		"description": "Zina restaurant - \"В гостях у Бабушки Зины\"",
		"id": "85d436f9-3500-4a4b-abea-840fd5e044ec",
		"image_url": "http://1385980-ci11141.tw1.ru:8080/images/zina.jpg",
		"location": {
			"latitude": 20.200000762939453,
			"longitude": 30.299999237060547
		},
		"menu": [],
		"name": "Zina",
		"opening_hours": "08:00-24:00",
		"price_range": 4,
		"rating": 4.320000171661377
	},
	{
		"address": "Mangilik el street, 29",
		"categories": [
			"FastFood"
		],
		"contact": "+7(777)133-45-67",
		"description": "Qazaq gourmet restaurant",
		"id": "344ce739-adb5-41da-8bd2-189db10cad39",
		"image_url": "http://1385980-ci11141.tw1.ru:8080/images/qazaq_gourmet.jpg",
		"location": {
			"latitude": 30.200000762939453,
			"longitude": 40.29999923706055
		},
		"menu": [],
		"name": "Qazaq Gourmet",
		"opening_hours": "12:00-23:00",
		"price_range": 4,
		"rating": 4.53000020980835
	},
	{
		"address": "Uly dala street, 45",
		"categories": [
			"Italian",
			"FastFood"
		],
		"contact": "+7(777)123-55-67",
		"description": "Halal restaurant",
		"id": "4d237a1c-0fd0-4f4a-b395-04b4468ecb14",
		"image_url": "http://1385980-ci11141.tw1.ru:8080/images/meat_point.png",
		"location": {
			"latitude": 50.20000076293945,
			"longitude": 60.29999923706055
		},
		"menu": [
			{
				"id": "7e4d2d2d-f1e9-406e-92fe-93d48ccd23ab",
				"image_url": "http://1385980-ci11141.tw1.ru:8080/images/burger.png",
				"ingredients": "Beef, salad, red onion, pickles, tomato",
				"menu_category": "FastFood",
				"name": "Hamburger",
				"price": 2600
			}
		],
		"name": "Meat point",
		"opening_hours": "00:00-24:00",
		"price_range": 3,
		"rating": 4.190000057220459
	},
	{
		"address": "Sarayshik street, 5",
		"categories": [],
		"contact": "+7(777)123-45-77",
		"description": "Chinese restaurant",
		"id": "006f41e5-ab90-4e86-9e74-c381e8220919",
		"image_url": "http://1385980-ci11141.tw1.ru:8080/images/lanzhou.png",
		"location": {
			"latitude": 50.20000076293945,
			"longitude": 60.29999923706055
		},
		"menu": [
			{
				"id": "03c972a3-d985-423a-ab34-848555d25e45",
				"image_url": "http://1385980-ci11141.tw1.ru:8080/images/Noodles.jpg",
				"ingredients": "Noodles, beef, onion, egg, soup",
				"menu_category": "Ramen",
				"name": "Noodles",
				"price": 2000
			}
		],
		"name": "Lanzhou",
		"opening_hours": "11:00-23:00",
		"price_range": 3,
		"rating": 4.400000095367432
	}
]
```

---

## Cities
```txt
GET /cities
```
Data:
```json
[
	{
		"ID": "569b6361-b229-4993-97c7-2ce90957d731",
		"Name": "Almaty"
	},
	{
		"ID": "67e7c525-461d-45f8-bb84-b53df464d48f",
		"Name": "Nur-Sultan"
	}
]
```

---

## Register
```txt
POST /register
```
Body:
```json
{
	"login": "user3",
	"password": "1234"
}
```
Response:
```json
{
	"login": "user3"
}
```

---

## Login
```txt
POST /login
```
Body:
```json
{
	"login": "user1",
	"password": "123"
}
```
Response:
```json
{
	"login": "user1"
}
```

---

## Categories
```txt
GET /categories
```
Data:
```json
[
	{
		"category": "FastFood",
		"image_url": "http://1385980-ci11141.tw1.ru:8080/images/fastfood_category.jpg",
		"restaurants": [
			{
				"address": "Mangilik el street, 29",
				"categories": [
					"FastFood"
				],
				"contact": "+7(777)133-45-67",
				"description": "Qazaq gourmet restaurant",
				"id": "344ce739-adb5-41da-8bd2-189db10cad39",
				"image_url": "http://1385980-ci11141.tw1.ru:8080/images/qazaq_gourmet.jpg",
				"location": {
					"latitude": 30.200000762939453,
					"longitude": 40.29999923706055
				},
				"menu": [],
				"name": "Qazaq Gourmet",
				"opening_hours": "12:00-23:00",
				"price_range": 4,
				"rating": 4.53000020980835
			},
			{
				"address": "Uly dala street, 45",
				"categories": [
					"Italian",
					"FastFood"
				],
				"contact": "+7(777)123-55-67",
				"description": "Halal restaurant",
				"id": "4d237a1c-0fd0-4f4a-b395-04b4468ecb14",
				"image_url": "http://1385980-ci11141.tw1.ru:8080/images/meat_point.png",
				"location": {
					"latitude": 50.20000076293945,
					"longitude": 60.29999923706055
				},
				"menu": [
					{
						"id": "f7e9ec4b-16cf-4baf-9d16-0c56af28b2ae",
						"image_url": "http://1385980-ci11141.tw1.ru:8080/images/burger.png",
						"ingredients": "Beef, salad, red onion, pickles, tomato",
						"menu_category": "FastFood",
						"name": "Hamburger",
						"price": 2600
					}
				],
				"name": "Meat point",
				"opening_hours": "00:00-24:00",
				"price_range": 3,
				"rating": 4.190000057220459
			}
		]
	},
	{
		"category": "Ramen",
		"image_url": "http://1385980-ci11141.tw1.ru:8080/images/ramen_category.jpg",
		"restaurants": [
			{
				"address": "Akyrtas street, 1/1",
				"categories": [
					"Ramen",
					"Japanese"
				],
				"contact": "+7(777)123-45-67",
				"description": "Zina restaurant - \"В гостях у Бабушки Зины\"",
				"id": "85d436f9-3500-4a4b-abea-840fd5e044ec",
				"image_url": "http://1385980-ci11141.tw1.ru:8080/images/zina.jpg",
				"location": {
					"latitude": 20.200000762939453,
					"longitude": 30.299999237060547
				},
				"menu": [],
				"name": "Zina",
				"opening_hours": "08:00-24:00",
				"price_range": 4,
				"rating": 4.320000171661377
			}
		]
	},
	{
		"category": "Italian",
		"image_url": "http://1385980-ci11141.tw1.ru:8080/images/italian_category.jpg",
		"restaurants": [
			{
				"address": "Uly dala street, 45",
				"categories": [
					"Italian",
					"FastFood"
				],
				"contact": "+7(777)123-55-67",
				"description": "Halal restaurant",
				"id": "4d237a1c-0fd0-4f4a-b395-04b4468ecb14",
				"image_url": "http://1385980-ci11141.tw1.ru:8080/images/meat_point.png",
				"location": {
					"latitude": 50.20000076293945,
					"longitude": 60.29999923706055
				},
				"menu": [
					{
						"id": "f7e9ec4b-16cf-4baf-9d16-0c56af28b2ae",
						"image_url": "http://1385980-ci11141.tw1.ru:8080/images/burger.png",
						"ingredients": "Beef, salad, red onion, pickles, tomato",
						"menu_category": "FastFood",
						"name": "Hamburger",
						"price": 2600
					}
				],
				"name": "Meat point",
				"opening_hours": "00:00-24:00",
				"price_range": 3,
				"rating": 4.190000057220459
			}
		]
	},
	{
		"category": "Japanese",
		"image_url": "http://1385980-ci11141.tw1.ru:8080/images/japanese_category.jpg",
		"restaurants": [
			{
				"address": "Akyrtas street, 1/1",
				"categories": [
					"Ramen",
					"Japanese"
				],
				"contact": "+7(777)123-45-67",
				"description": "Zina restaurant - \"В гостях у Бабушки Зины\"",
				"id": "85d436f9-3500-4a4b-abea-840fd5e044ec",
				"image_url": "http://1385980-ci11141.tw1.ru:8080/images/zina.jpg",
				"location": {
					"latitude": 20.200000762939453,
					"longitude": 30.299999237060547
				},
				"menu": [],
				"name": "Zina",
				"opening_hours": "08:00-24:00",
				"price_range": 4,
				"rating": 4.320000171661377
			}
		]
	}
]
```

---

## Locations
```txt
GET /locations?longitude=50.00&latitude=60.00
```
Data:
```json
[
	{
		"address": "Akyrtas street, 1/1",
		"categories": [
			"Ramen",
			"Japanese"
		],
		"contact": "+7(777)123-45-67",
		"description": "Zina restaurant - \"В гостях у Бабушки Зины\"",
		"id": "85d436f9-3500-4a4b-abea-840fd5e044ec",
		"image_url": "http://1385980-ci11141.tw1.ru:8080/images/zina.jpg",
		"location": {
			"latitude": 20.200000762939453,
			"longitude": 30.299999237060547
		},
		"menu": [],
		"name": "Zina",
		"opening_hours": "08:00-24:00",
		"price_range": 4,
		"rating": 4.320000171661377
	},
	{
		"address": "Mangilik el street, 29",
		"categories": [
			"FastFood"
		],
		"contact": "+7(777)133-45-67",
		"description": "Qazaq gourmet restaurant",
		"id": "344ce739-adb5-41da-8bd2-189db10cad39",
		"image_url": "http://1385980-ci11141.tw1.ru:8080/images/qazaq_gourmet.jpg",
		"location": {
			"latitude": 30.200000762939453,
			"longitude": 40.29999923706055
		},
		"menu": [],
		"name": "Qazaq Gourmet",
		"opening_hours": "12:00-23:00",
		"price_range": 4,
		"rating": 4.53000020980835
	},
	{
		"address": "Uly dala street, 45",
		"categories": [
			"Italian",
			"FastFood"
		],
		"contact": "+7(777)123-55-67",
		"description": "Halal restaurant",
		"id": "4d237a1c-0fd0-4f4a-b395-04b4468ecb14",
		"image_url": "http://1385980-ci11141.tw1.ru:8080/images/meat_point.png",
		"location": {
			"latitude": 50.20000076293945,
			"longitude": 60.29999923706055
		},
		"menu": [
			{
				"id": "7e4d2d2d-f1e9-406e-92fe-93d48ccd23ab",
				"image_url": "http://1385980-ci11141.tw1.ru:8080/images/burger.png",
				"ingredients": "Beef, salad, red onion, pickles, tomato",
				"menu_category": "FastFood",
				"name": "Hamburger",
				"price": 2600
			}
		],
		"name": "Meat point",
		"opening_hours": "00:00-24:00",
		"price_range": 3,
		"rating": 4.190000057220459
	},
	{
		"address": "Sarayshik street, 5",
		"categories": [],
		"contact": "+7(777)123-45-77",
		"description": "Chinese restaurant",
		"id": "006f41e5-ab90-4e86-9e74-c381e8220919",
		"image_url": "http://1385980-ci11141.tw1.ru:8080/images/lanzhou.png",
		"location": {
			"latitude": 50.20000076293945,
			"longitude": 60.29999923706055
		},
		"menu": [
			{
				"id": "03c972a3-d985-423a-ab34-848555d25e45",
				"image_url": "http://1385980-ci11141.tw1.ru:8080/images/Noodles.jpg",
				"ingredients": "Noodles, beef, onion, egg, soup",
				"menu_category": "Ramen",
				"name": "Noodles",
				"price": 2000
			}
		],
		"name": "Lanzhou",
		"opening_hours": "11:00-23:00",
		"price_range": 3,
		"rating": 4.400000095367432
	}
]
```

---

## Favorites
```txt
POST /favorite
```
Body:
```json
{
	"login": "user1",
	"restaurant_id": "006f41e5-ab90-4e86-9e74-c381e8220919"
}
```
Response:
```json
{
	"message": "ok"
}
```

---

```txt
DELETE /delete_favorite
```
Body:
```json
{
	"login": "user1",
	"restaurant_id": "006f41e5-ab90-4e86-9e74-c381e8220919"
}
```
Response:
```json
{
	"message": "ok"
}
```

---

```txt
POST /get_favorites
```
Body:
```json
{
	"login": "user1"
}
```
Response:
```json
[
	{
		"address": "Akyrtas street, 1/1",
		"categories": [
			"Ramen",
			"Japanese"
		],
		"contact": "+7(777)123-45-67",
		"description": "Zina restaurant - \"В гостях у Бабушки Зины\"",
		"id": "85d436f9-3500-4a4b-abea-840fd5e044ec",
		"image_url": "http://1385980-ci11141.tw1.ru:8080/images/zina.jpg",
		"location": {
			"latitude": 20.200000762939453,
			"longitude": 30.299999237060547
		},
		"menu": [],
		"name": "Zina",
		"opening_hours": "08:00-24:00",
		"price_range": 4,
		"rating": 4.320000171661377
	}
]
```

---

## Reservation

```txt
POST /reserve
```
Body:
```json
{
	"table_id": "32569054-c18a-4815-8b0b-1f3d33472ecc",
	"user_id": "user1",
	"personas": 3,
	"reservation_start": "2015-06-03T18:00:00.789",
	"reservation_end": "2015-06-03T20:00:00.789"
}
```
Response:
```json
{
	"message": "ok"
}
```
---

```txt
DELETE /delete_reservation
```
Body:
```json
{
	"login": "user1",
	"reservation_id": "73a829d5-dc3b-4d2a-b95f-93f13b25c9c2",
	"table_id": "32569054-c18a-4815-8b0b-1f3d33472ecc"
}```
Response:
```json
{
	"message": "ok"
}
```

---

```txt
POST /get_reservations
```
Body:
```json
{
	"login": "user1"
}
```
Response:
```json
[
	{
		"reservation": {
			"id": "b021e5ad-1448-490e-98ec-605a69c484f9",
			"personas": 2,
			"reservation_end": "2015-06-03T20:00:00.789",
			"reservation_start": "2015-06-03T18:00:00.789",
			"table_id": "ae345dc4-e181-4747-a964-f28cd98a20cd",
			"user_id": "user1"
		},
		"restaurant": {
			"address": "Akyrtas street, 1/1",
			"categories": [
				"Ramen",
				"Japanese"
			],
			"contact": "+7(777)123-45-67",
			"description": "Zina restaurant - \"В гостях у Бабушки Зины\"",
			"id": "85d436f9-3500-4a4b-abea-840fd5e044ec",
			"image_url": "http://1385980-ci11141.tw1.ru:8080/images/zina.jpg",
			"location": {
				"latitude": 20.200000762939453,
				"longitude": 30.299999237060547
			},
			"menu": [],
			"name": "Zina",
			"opening_hours": "08:00-24:00",
			"price_range": 4,
			"rating": 4.320000171661377
		}
	}
]
```
---

## Tables

```txt
GET /{restaurant_id}/tables
```
Response:
```json
[
	{
		"id": "0657b480-9484-4a4e-8f92-13b9193911d2",
		"restaurant_info_id": "85d436f9-3500-4a4b-abea-840fd5e044ec",
		"seats": 2,
		"status": "Free"
	},
	{
		"id": "dc08149a-6fc5-4373-9464-037ec7a4ab3c",
		"restaurant_info_id": "85d436f9-3500-4a4b-abea-840fd5e044ec",
		"seats": 2,
		"status": "Free"
	},
	{
		"id": "f685aa62-66fa-4fb7-8065-8cb531a318e3",
		"restaurant_info_id": "85d436f9-3500-4a4b-abea-840fd5e044ec",
		"seats": 4,
		"status": "Free"
	},
	{
		"id": "ae345dc4-e181-4747-a964-f28cd98a20cd",
		"restaurant_info_id": "85d436f9-3500-4a4b-abea-840fd5e044ec",
		"seats": 2,
		"status": "Occupied"
	}
]
```
