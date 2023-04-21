# Endpoints

## Restaurants
```txt
GET /restaurants
```
Data:
```json
[
	{
		"categories": [
			"Ramen",
			"Japanese"
		],
		"info": {
			"address": "Akyrtas street, 1/1",
			"contact": "+7(777)123-45-67",
			"description": "Zina restaurant - \"В гостях у Бабушки Зины\"",
			"id": "85d436f9-3500-4a4b-abea-840fd5e044ec",
			"image_url": "http://1385980-ci11141.tw1.ru:8080/images/zina.jpg",
			"name": "Zina",
			"opening_hours": "08:00-24:00",
			"price_range": 4,
			"rating": 4.320000171661377
		},
		"location": {
			"latitude": 20.200000762939453,
			"longitude": 30.299999237060547
		},
		"menu": []
	},
	{
		"categories": [
			"FastFood"
		],
		"info": {
			"address": "Mangilik el street, 29",
			"contact": "+7(777)133-45-67",
			"description": "Qazaq gourmet restaurant",
			"id": "344ce739-adb5-41da-8bd2-189db10cad39",
			"image_url": "http://1385980-ci11141.tw1.ru:8080/images/qazaq_gourmet.jpg",
			"name": "Qazaq Gourmet",
			"opening_hours": "12:00-23:00",
			"price_range": 4,
			"rating": 4.53000020980835
		},
		"location": {
			"latitude": 30.200000762939453,
			"longitude": 40.29999923706055
		},
		"menu": []
	},
	{
		"categories": [
			"Italian",
			"FastFood"
		],
		"info": {
			"address": "Uly dala street, 45",
			"contact": "+7(777)123-55-67",
			"description": "Halal restaurant",
			"id": "4d237a1c-0fd0-4f4a-b395-04b4468ecb14",
			"image_url": "http://1385980-ci11141.tw1.ru:8080/images/meat_point.png",
			"name": "Meat point",
			"opening_hours": "00:00-24:00",
			"price_range": 3,
			"rating": 4.190000057220459
		},
		"location": {
			"latitude": 50.20000076293945,
			"longitude": 60.29999923706055
		},
		"menu": [
			{
				"id": "371e9375-6205-47f1-b64c-08ab5733a75f",
				"image_url": "http://1385980-ci11141.tw1.ru:8080/images/burger.png",
				"ingredients": "Beef, salad, red onion, pickles, tomato",
				"menu_category": "FastFood",
				"name": "Hamburger",
				"price": 2600
			}
		]
	},
	{
		"categories": [],
		"info": {
			"address": "Sarayshik street, 5",
			"contact": "+7(777)123-45-77",
			"description": "Chinese restaurant",
			"id": "006f41e5-ab90-4e86-9e74-c381e8220919",
			"image_url": "http://1385980-ci11141.tw1.ru:8080/images/lanzhou.png",
			"name": "Lanzhou",
			"opening_hours": "11:00-23:00",
			"price_range": 3,
			"rating": 4.400000095367432
		},
		"location": {
			"latitude": 50.20000076293945,
			"longitude": 60.29999923706055
		},
		"menu": [
			{
				"id": "aee9b8f8-48da-4925-a7bd-3bb7603ccad1",
				"image_url": "http://1385980-ci11141.tw1.ru:8080/images/Noodles.jpg",
				"ingredients": "Noodles, beef, onion, egg, soup",
				"menu_category": "Ramen",
				"name": "Noodles",
				"price": 2000
			}
		]
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
		"category": "Japanese",
		"restaurants": [
			{
				"categories": [
					"Ramen",
					"Japanese"
				],
				"info": {
					"address": "Akyrtas street, 1/1",
					"contact": "+7(777)123-45-67",
					"description": "Zina restaurant - \"В гостях у Бабушки Зины\"",
					"id": "85d436f9-3500-4a4b-abea-840fd5e044ec",
					"image_url": "http://1385980-ci11141.tw1.ru:8080/images/zina.jpg",
					"name": "Zina",
					"opening_hours": "08:00-24:00",
					"price_range": 4,
					"rating": 4.320000171661377
				},
				"location": {
					"latitude": 20.200000762939453,
					"longitude": 30.299999237060547
				},
				"menu": []
			}
		]
	},
	{
		"category": "FastFood",
		"restaurants": [
			{
				"categories": [
					"FastFood"
				],
				"info": {
					"address": "Mangilik el street, 29",
					"contact": "+7(777)133-45-67",
					"description": "Qazaq gourmet restaurant",
					"id": "344ce739-adb5-41da-8bd2-189db10cad39",
					"image_url": "http://1385980-ci11141.tw1.ru:8080/images/qazaq_gourmet.jpg",
					"name": "Qazaq Gourmet",
					"opening_hours": "12:00-23:00",
					"price_range": 4,
					"rating": 4.53000020980835
				},
				"location": {
					"latitude": 30.200000762939453,
					"longitude": 40.29999923706055
				},
				"menu": []
			},
			{
				"categories": [
					"Italian",
					"FastFood"
				],
				"info": {
					"address": "Uly dala street, 45",
					"contact": "+7(777)123-55-67",
					"description": "Halal restaurant",
					"id": "4d237a1c-0fd0-4f4a-b395-04b4468ecb14",
					"image_url": "http://1385980-ci11141.tw1.ru:8080/images/meat_point.png",
					"name": "Meat point",
					"opening_hours": "00:00-24:00",
					"price_range": 3,
					"rating": 4.190000057220459
				},
				"location": {
					"latitude": 50.20000076293945,
					"longitude": 60.29999923706055
				},
				"menu": [
					{
						"id": "371e9375-6205-47f1-b64c-08ab5733a75f",
						"image_url": "http://1385980-ci11141.tw1.ru:8080/images/burger.png",
						"ingredients": "Beef, salad, red onion, pickles, tomato",
						"menu_category": "FastFood",
						"name": "Hamburger",
						"price": 2600
					}
				]
			}
		]
	},
	{
		"category": "Italian",
		"restaurants": [
			{
				"categories": [
					"Italian",
					"FastFood"
				],
				"info": {
					"address": "Uly dala street, 45",
					"contact": "+7(777)123-55-67",
					"description": "Halal restaurant",
					"id": "4d237a1c-0fd0-4f4a-b395-04b4468ecb14",
					"image_url": "http://1385980-ci11141.tw1.ru:8080/images/meat_point.png",
					"name": "Meat point",
					"opening_hours": "00:00-24:00",
					"price_range": 3,
					"rating": 4.190000057220459
				},
				"location": {
					"latitude": 50.20000076293945,
					"longitude": 60.29999923706055
				},
				"menu": [
					{
						"id": "371e9375-6205-47f1-b64c-08ab5733a75f",
						"image_url": "http://1385980-ci11141.tw1.ru:8080/images/burger.png",
						"ingredients": "Beef, salad, red onion, pickles, tomato",
						"menu_category": "FastFood",
						"name": "Hamburger",
						"price": 2600
					}
				]
			}
		]
	},
	{
		"category": "Ramen",
		"restaurants": [
			{
				"categories": [
					"Ramen",
					"Japanese"
				],
				"info": {
					"address": "Akyrtas street, 1/1",
					"contact": "+7(777)123-45-67",
					"description": "Zina restaurant - \"В гостях у Бабушки Зины\"",
					"id": "85d436f9-3500-4a4b-abea-840fd5e044ec",
					"image_url": "http://1385980-ci11141.tw1.ru:8080/images/zina.jpg",
					"name": "Zina",
					"opening_hours": "08:00-24:00",
					"price_range": 4,
					"rating": 4.320000171661377
				},
				"location": {
					"latitude": 20.200000762939453,
					"longitude": 30.299999237060547
				},
				"menu": []
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
		"categories": [
			"Ramen",
			"Japanese"
		],
		"info": {
			"address": "Akyrtas street, 1/1",
			"contact": "+7(777)123-45-67",
			"description": "Zina restaurant - \"В гостях у Бабушки Зины\"",
			"id": "85d436f9-3500-4a4b-abea-840fd5e044ec",
			"image_url": "http://1385980-ci11141.tw1.ru:8080/images/zina.jpg",
			"name": "Zina",
			"opening_hours": "08:00-24:00",
			"price_range": 4,
			"rating": 4.320000171661377
		},
		"location": {
			"latitude": 20.200000762939453,
			"longitude": 30.299999237060547
		},
		"menu": []
	},
	{
		"categories": [
			"FastFood"
		],
		"info": {
			"address": "Mangilik el street, 29",
			"contact": "+7(777)133-45-67",
			"description": "Qazaq gourmet restaurant",
			"id": "344ce739-adb5-41da-8bd2-189db10cad39",
			"image_url": "http://1385980-ci11141.tw1.ru:8080/images/qazaq_gourmet.jpg",
			"name": "Qazaq Gourmet",
			"opening_hours": "12:00-23:00",
			"price_range": 4,
			"rating": 4.53000020980835
		},
		"location": {
			"latitude": 30.200000762939453,
			"longitude": 40.29999923706055
		},
		"menu": []
	},
	{
		"categories": [
			"Italian",
			"FastFood"
		],
		"info": {
			"address": "Uly dala street, 45",
			"contact": "+7(777)123-55-67",
			"description": "Halal restaurant",
			"id": "4d237a1c-0fd0-4f4a-b395-04b4468ecb14",
			"image_url": "http://1385980-ci11141.tw1.ru:8080/images/meat_point.png",
			"name": "Meat point",
			"opening_hours": "00:00-24:00",
			"price_range": 3,
			"rating": 4.190000057220459
		},
		"location": {
			"latitude": 50.20000076293945,
			"longitude": 60.29999923706055
		},
		"menu": [
			{
				"id": "371e9375-6205-47f1-b64c-08ab5733a75f",
				"image_url": "http://1385980-ci11141.tw1.ru:8080/images/burger.png",
				"ingredients": "Beef, salad, red onion, pickles, tomato",
				"menu_category": "FastFood",
				"name": "Hamburger",
				"price": 2600
			}
		]
	},
	{
		"categories": [],
		"info": {
			"address": "Sarayshik street, 5",
			"contact": "+7(777)123-45-77",
			"description": "Chinese restaurant",
			"id": "006f41e5-ab90-4e86-9e74-c381e8220919",
			"image_url": "http://1385980-ci11141.tw1.ru:8080/images/lanzhou.png",
			"name": "Lanzhou",
			"opening_hours": "11:00-23:00",
			"price_range": 3,
			"rating": 4.400000095367432
		},
		"location": {
			"latitude": 50.20000076293945,
			"longitude": 60.29999923706055
		},
		"menu": [
			{
				"id": "aee9b8f8-48da-4925-a7bd-3bb7603ccad1",
				"image_url": "http://1385980-ci11141.tw1.ru:8080/images/Noodles.jpg",
				"ingredients": "Noodles, beef, onion, egg, soup",
				"menu_category": "Ramen",
				"name": "Noodles",
				"price": 2000
			}
		]
	}
]
```

---

## Favorites
```txt
POST /add_favorite
```
Body:
```json
{
	"login": "user1",
	"restaurant_id": "006f41e5-ab90-4e86-9e74-c381e8220919"
}
```
Response: 200 OK

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
Response: 200 OK

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
		"categories": [
			"Ramen",
			"Japanese"
		],
		"info": {
			"address": "Akyrtas street, 1/1",
			"contact": "+7(777)123-45-67",
			"description": "Zina restaurant - \"В гостях у Бабушки Зины\"",
			"id": "85d436f9-3500-4a4b-abea-840fd5e044ec",
			"image_url": "http://1385980-ci11141.tw1.ru:8080/images/zina.jpg",
			"name": "Zina",
			"opening_hours": "08:00-24:00",
			"price_range": 4,
			"rating": 4.320000171661377
		},
		"location": {
			"latitude": 20.200000762939453,
			"longitude": 30.299999237060547
		},
		"menu": []
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
Response: 200 OK

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
Response: 200 OK

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
			"id": "8313b249-b355-4dcf-b536-b70b0dd7b417",
			"personas": 3,
			"reservation_end": "2015-06-03T20:00:00.789",
			"reservation_start": "2015-06-03T18:00:00.789",
			"table_id": "42c35e08-028a-41c7-81cd-3abb3f6b83da",
			"user_id": "user1"
		},
		"restaurant": {
			"categories": [
				"Ramen",
				"Japanese"
			],
			"info": {
				"address": "Akyrtas street, 1/1",
				"contact": "+7(777)123-45-67",
				"description": "Zina restaurant - \"В гостях у Бабушки Зины\"",
				"id": "85d436f9-3500-4a4b-abea-840fd5e044ec",
				"image_url": "http://1385980-ci11141.tw1.ru:8080/images/zina.jpg",
				"name": "Zina",
				"opening_hours": "08:00-24:00",
				"price_range": 4,
				"rating": 4.320000171661377
			},
			"location": {
				"latitude": 20.200000762939453,
				"longitude": 30.299999237060547
			},
			"menu": []
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
		"restaurant_info_id": "85d436f9-3500-4a4b-abea-840fd5e044ec",
		"seats": 2,
		"status": "Occupied",
		"table_id": 1
	},
	{
		"restaurant_info_id": "85d436f9-3500-4a4b-abea-840fd5e044ec",
		"seats": 2,
		"status": "Occupied",
		"table_id": 2
	},
	{
		"restaurant_info_id": "85d436f9-3500-4a4b-abea-840fd5e044ec",
		"seats": 4,
		"status": "Free",
		"table_id": 3
	},
	{
		"restaurant_info_id": "85d436f9-3500-4a4b-abea-840fd5e044ec",
		"seats": 2,
		"status": "Occupied",
		"table_id": 4
	}
]
```
