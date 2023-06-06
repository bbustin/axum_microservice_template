-- Up: Initial Tasks table
CREATE TABLE tasks
(
    id INTEGER PRIMARY KEY,
    task varchar(255) NOT NULL
);

INSERT INTO tasks
    (id, task)
VALUES
    (1, "Read README"),
    (2, "See if it runs"),
    (3, "Remove example code and get started")
;