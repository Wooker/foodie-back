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
		"coordinate": {
			"latitude": 20.200000762939453,
			"longitude": 30.299999237060547
		},
		"description": "Zina restaurant - \"В гостях у Бабушки Зины\"",
		"id": "85d436f9-3500-4a4b-abea-840fd5e044ec",
		"image_url": "http://1385980-ci11141.tw1.ru:8080/images/zina.jpg",
		"menu_items": [],
		"name": "Zina",
		"opening_hours": "08:00-24:00",
		"price_range": 4
	},
	{
		"address": "Mangilik el street, 29",
		"categories": [
			"FastFood"
		],
		"coordinate": {
			"latitude": 30.200000762939453,
			"longitude": 40.29999923706055
		},
		"description": "Qazaq gourmet restaurant",
		"id": "344ce739-adb5-41da-8bd2-189db10cad39",
		"image_url": "http://1385980-ci11141.tw1.ru:8080/images/qazaq_gourmet.jpg",
		"menu_items": [],
		"name": "Qazaq Gourmet",
		"opening_hours": "12:00-23:00",
		"price_range": 4
	},
	{
		"address": "Uly dala street, 45",
		"categories": [
			"Italian",
			"FastFood"
		],
		"coordinate": {
			"latitude": 50.20000076293945,
			"longitude": 60.29999923706055
		},
		"description": "Halal restaurant",
		"id": "4d237a1c-0fd0-4f4a-b395-04b4468ecb14",
		"image_url": "http://1385980-ci11141.tw1.ru:8080/images/meat_point.png",
		"menu_items": [
			{
				"id": "a4c5e8cf-71bb-45a6-b963-d043263daa37",
				"image_url": "http://1385980-ci11141.tw1.ru:8080/images/burger.png",
				"ingredients": "Beef, salad, red onion, pickles, tomato",
				"menu_category": "FastFood",
				"name": "Hamburger",
				"price": 2600
			}
		],
		"name": "Meat point",
		"opening_hours": "00:00-24:00",
		"price_range": 3
	},
	{
		"address": "Sarayshik street, 5",
		"categories": [],
		"coordinate": {
			"latitude": 50.20000076293945,
			"longitude": 60.29999923706055
		},
		"description": "Chinese restaurant",
		"id": "006f41e5-ab90-4e86-9e74-c381e8220919",
		"image_url": "http://1385980-ci11141.tw1.ru:8080/images/lanzhou.png",
		"menu_items": [
			{
				"id": "54f14dbc-b1ca-49de-9dd8-c276b2cdced6",
				"image_url": "http://1385980-ci11141.tw1.ru:8080/images/Noodles.jpg",
				"ingredients": "Noodles, beef, onion, egg, soup",
				"menu_category": "Ramen",
				"name": "Noodles",
				"price": 2000
			}
		],
		"name": "Lanzhou",
		"opening_hours": "11:00-23:00",
		"price_range": 3
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
[{
    "category": "Ramen",
    "restaurants": [{
        "address": "Akyrtas street, 1/1",
        "averagePrice": "10000",
        "description": "Zina restaurant - \"В гостях у Бабушки Зины\"",
        "id": "85d436f9-3500-4a4b-abea-840fd5e044ec",
        "image_url": "http://1140427-ci11141.tw1.ru:8080/images/zina.jpg",
        "name": "Zina",
        "openHours": "08:00-24:00"
    }]
}, {
    "category": "Italian",
    "restaurants": [{
        "address": "Uly dala street, 45",
        "averagePrice": "4000",
        "description": "Halal restaurant",
        "id": "4d237a1c-0fd0-4f4a-b395-04b4468ecb14",
        "image_url": "http://1140427-ci11141.tw1.ru:8080/images/meat_point.png",
        "name": "Meat point",
        "openHours": "00:00-24:00"
    }]
}, {
    "category": "FastFood",
    "restaurants": [{
        "address": "Mangilik el street, 29",
        "averagePrice": "20000",
        "description": "Qazaq gourmet restaurant",
        "id": "344ce739-adb5-41da-8bd2-189db10cad39",
        "image_url": "http://1140427-ci11141.tw1.ru:8080/images/qazaq_gourmet.jpg",
        "name": "Qazaq Gourmet",
        "openHours": "12:00-23:00"
    }, {
        "address": "Uly dala street, 45",
        "averagePrice": "4000",
        "description": "Halal restaurant",
        "id": "4d237a1c-0fd0-4f4a-b395-04b4468ecb14",
        "image_url": "http://1140427-ci11141.tw1.ru:8080/images/meat_point.png",
        "name": "Meat point",
        "openHours": "00:00-24:00"
    }]
}, {
    "category": "Japanese",
    "restaurants": [{
        "address": "Akyrtas street, 1/1",
        "averagePrice": "10000",
        "description": "Zina restaurant - \"В гостях у Бабушки Зины\"",
        "id": "85d436f9-3500-4a4b-abea-840fd5e044ec",
        "image_url": "http://1140427-ci11141.tw1.ru:8080/images/zina.jpg",
        "name": "Zina",
        "openHours": "08:00-24:00"
    }]
}]
```

---

## Locations
```txt
GET /locations?longitude=50.00&latitude=60.00
```
Data:
```json
[{
    "location": {
        "latitude": 60.29999923706055,
        "longitude": 50.20000076293945
    },
    "restaurant": {
        "address": "Uly dala street, 45",
        "averagePrice": "4000",
        "description": "Halal restaurant",
        "id": "4d237a1c-0fd0-4f4a-b395-04b4468ecb14",
        "image_url": "http://1140427-ci11141.tw1.ru:8080/images/meat_point.png",
        "name": "Meat point",
        "openHours": "00:00-24:00"
    }
}, {
    "location": {
        "latitude": 40.29999923706055,
        "longitude": 30.200000762939453
    },
    "restaurant": {
        "address": "Mangilik el street, 29",
        "averagePrice": "20000",
        "description": "Qazaq gourmet restaurant",
        "id": "344ce739-adb5-41da-8bd2-189db10cad39",
        "image_url": "http://1140427-ci11141.tw1.ru:8080/images/qazaq_gourmet.jpg",
        "name": "Qazaq Gourmet",
        "openHours": "12:00-23:00"
    }
}, {
    "location": {
        "latitude": 30.299999237060547,
        "longitude": 20.200000762939453
    },
    "restaurant": {
        "address": "Akyrtas street, 1/1",
        "averagePrice": "10000",
        "description": "Zina restaurant - \"В гостях у Бабушки Зины\"",
        "id": "85d436f9-3500-4a4b-abea-840fd5e044ec",
        "image_url": "http://1140427-ci11141.tw1.ru:8080/images/zina.jpg",
        "name": "Zina",
        "openHours": "08:00-24:00"
    }
}]
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
["85d436f9-3500-4a4b-abea-840fd5e044ec", "006f41e5-ab90-4e86-9e74-c381e8220919"]
```
---

## Reservation
```txt
POST /reserve
```
Body:
```json
{
	"login": "user1",
	"restaurant_id": "006f41e5-ab90-4e86-9e74-c381e8220919",
	"table_id": 1
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
	"restaurant_id": "006f41e5-ab90-4e86-9e74-c381e8220919",
	"table_id": 1
}
```
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
		"restaurant_id": "006f41e5-ab90-4e86-9e74-c381e8220919",
		"table_id": 1
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
