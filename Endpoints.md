# Endpoints

## Restaurants
```txt
GET /restaurants
```
Data:
```json
[
	{
		"Address": "Address1",
		"AveragePrice": "1000",
		"Description": "Description1",
		"ID": "85d436f9-3500-4a4b-abea-840fd5e044ec",
		"Images": "",
		"Name": "Name1",
		"OpenHours": "OpenHours1"
	},
	{
		"Address": "Address2",
		"AveragePrice": "2000",
		"Description": "Description2",
		"ID": "344ce739-adb5-41da-8bd2-189db10cad39",
		"Images": "",
		"Name": "Name2",
		"OpenHours": "OpenHours2"
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
	"login": "user",
	"password": "password"
}
```

## Login
```txt
POST /login
```
Body:
```json
{
	"login": "user",
	"password": "password"
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
