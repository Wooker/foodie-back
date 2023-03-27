# Endpoints

## Restaurants
```txt
GET /restaurants
```
Data:
```json
[
	{
		"address": "Address1",
		"averagePrice": "1000",
		"description": "Description1",
		"id": "85d436f9-3500-4a4b-abea-840fd5e044ec",
		"images": "",
		"name": "Name1",
		"openHours": "OpenHours1"
	},
	{
		"address": "Address2",
		"averagePrice": "2000",
		"description": "Description2",
		"id": "344ce739-adb5-41da-8bd2-189db10cad39",
		"images": "",
		"name": "Name2",
		"openHours": "OpenHours2"
	}
]
```

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
## Categories
```txt
GET /categories
```
Data:
```json
[
  {
    "category": "FastFood",
    "restaurants": [
      {
        "address": "Address2",
        "averagePrice": "2000",
        "description": "Description2",
        "id": "344ce739-adb5-41da-8bd2-189db10cad39",
        "images": "",
        "name": "Name2",
        "openHours": "OpenHours2"
      },
      {
        "address": "Address3",
        "averagePrice": "3000",
        "description": "Description3",
        "id": "4d237a1c-0fd0-4f4a-b395-04b4468ecb14",
        "images": "",
        "name": "Name3",
        "openHours": "OpenHours3"
      }
    ]
  }
]
```

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
        "address": "Address3",
        "averagePrice": "3000",
        "description": "Description3",
        "id": "4d237a1c-0fd0-4f4a-b395-04b4468ecb14",
        "images": "",
        "name": "Name3",
        "openHours": "OpenHours3"
    }
}, {
    "location": {
        "latitude": 40.29999923706055,
        "longitude": 30.200000762939453
    },
    "restaurant": {
        "address": "Address2",
        "averagePrice": "2000",
        "description": "Description2",
        "id": "344ce739-adb5-41da-8bd2-189db10cad39",
        "images": "",
        "name": "Name2",
        "openHours": "OpenHours2"
    }
}]
```
