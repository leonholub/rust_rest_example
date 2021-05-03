CREATE TABLE vehicles (
    id SERIAL PRIMARY KEY,
    name VARCHAR(100) NOT NULL,
    coolness INT NOT NULL,
    wattage INT NOT NULL,
    description TEXT
);