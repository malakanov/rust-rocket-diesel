CREATE TABLE posts (
                       id INTEGER PRIMARY KEY,
                       title VARCHAR NOT NULL,
                       body TEXT NOT NULL,
                       genre VARCHAR NOT NULL,
                       published BOOLEAN NOT NULL DEFAULT false
)
