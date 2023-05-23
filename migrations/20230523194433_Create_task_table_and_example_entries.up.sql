-- Up: Initial Tasks table
CREATE TABLE task (
    id  INTEGER PRIMARY KEY,
    task varchar(255) NOT NULL
);

INSERT INTO task (id, task)
VALUES
    (1, "Read README"),
    (2, "See if it runs"),
    (3, "Remove example code and get started")
;