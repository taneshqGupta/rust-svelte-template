CREATE TABLE todos (
    id SERIAL PRIMARY KEY,
    descript TEXT NOT NULL,
    done BOOLEAN NOT NULL DEFAULT FALSE
);