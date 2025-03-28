CREATE TABLE partidos (
id SERIAL PRIMARY KEY,
equipo_local VARCHAR(100) NOT NULL,
equipo_visitante VARCHAR(100) NOT NULL,
date TIMESTAMP 
);