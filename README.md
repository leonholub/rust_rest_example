# Restful rusty rocket example
Example application to show the use of the rust web framework rocket 
by building a simple rest API.

Features:
- rocket
- diesel
- Basic REST API (GET, POST)
- Login
- JWT Authentication
- Password Hashing
 
 
 ## Diesel Setup
 Make sure to have set the database connection in the `DATABASE_URL` 
 environment variable like this `export DATABASE_URL=postgres://<user>:<password>@localhost/<database>`
 
 1. `diesel setup`
 2. `diesel migration generate <name>`
 3. Edit migration
 4. `diesel migration run`
 
## Diesel commands
- reset migration `diesel migration redo`

## Curl tips

```
curl --header "Content-Type: application/json" \
  --request POST \
  --data '{"name":"Bike","coolness":100,"wattage":8000,"description":"The coolest thing in the world"}' \
  http://localhost:8000/api/vehicle

curl --header "Content-Type: application/json" \
  --request PUT \
  --data '{"id":2, "name":"Roadbike","coolness":101,"wattage":8000,"description":"The coolest thing in the world"}' \
  http://localhost:8000/api/vehicle/2

```