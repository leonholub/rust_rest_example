# Restful rusty rocket example
Example application to test the use of the rust web framework rocket by building a simple rest API with JWT Auth.
Since it bothered me to find so many example applications online with either hard-coded API keys or user databases 
with plaintext passwords i decided to do a simple one myself to try out a few techniques.
Application is by no means complete or secure, this is just some example and notes on several things.
Obligatory don't copy-paste this and use it in production or anywhere public.

Feedback is welcome and appreciated.

Features:
- rocket
- diesel
- rust crypto (crates crypt-mac, argon2)
- Basic REST API (GET, POST)
    - Login Endpoint
    - JWT Authentication
    - example data endpoint
 
 ## Open TODOs
 
 - Improve Documentation
 - Clean up code
 - Reorganize dependencies and improve modularity
 
 ## Database Connection - Diesel
 
 - https://diesel.rs/guides/getting-started.html
 - https://docs.rs/diesel/1.4.6/diesel/index.html
 - https://docs.diesel.rs/diesel/query_dsl/trait.QueryDsl.html#method.filter

 ### Setup
 Spin up your favourite diesel-compatible db engine. I use postgreSQL in this example.
 
 Make sure to have set the database connection in the `DATABASE_URL` 
 environment variable like this `export DATABASE_URL=postgres://<user>:<password>@localhost/<database>`
 
 And do the following: 
 
 1. Generate project files`diesel setup`
 2. Generate migration files `diesel migration generate <name>`
 3. Edit migration files
 4. Apply migration `diesel migration run`
 
 Optionally run `diesel migration redo` to test if your `down.sql` file works, too.

`
psql -U root webtest
`
## Environment variables
```
APPLICATION_SECRET - A secret application key used for jwt. Should be random and strong
DATABASE_URL - The url to the postgresql database
```

## Curl commands

Some curl commands to test the API locally

```
curl --header "Content-Type: application/json" \
  --request POST \
  --data '{"name":"Bike","coolness":100,"wattage":8000,"description":"The coolest thing in the world"}' \
  http://localhost:8000/api/vehicle

curl --header "Content-Type: application/json" \
  --request PUT \
  --data '{"id":2, "name":"Roadbike","coolness":101,"wattage":8000,"description":"The coolest thing in the world"}' \
  http://localhost:8000/api/vehicle/2

curl --header "Authentication: XYZ-token" --request GET -I http://localhost:8000/

curl --header "Content-Type: application/json" \
  --request POST \
  --data '{"username":"leon", "password": "xyz"}' \
  http://localhost:8000/login

```

## Reading List
- https://genekuo.medium.com/creating-a-rest-api-in-rust-with-persistence-rust-rocket-and-diesel-a4117d400104
- https://medium.com/digitalfrontiers/web-service-with-rust-rocket-and-diesel-7425f4a04f4c
- https://medium.com/@james_32022/authentication-in-rocket-feb4f7223254
- https://lankydan.dev/2018/05/20/creating-a-rusty-rocket-fuelled-with-diesel
- https://docs.rs/jwt/0.13.0/jwt/#signing
- https://docs.rs/argon2/0.2.0/argon2/
- https://rocket.rs/v0.4/guide/requests/#custom-guards
- https://github.com/lankydan/rust-web-with-rocket/blob/master/src/people/handler.rs
- https://github.com/sean3z/rocket-diesel-rest-api-example/blob/master/src/main.rs
- https://www.arewewebyet.org/
- https://github.com/hgzimmerman/FullstackRustDemo