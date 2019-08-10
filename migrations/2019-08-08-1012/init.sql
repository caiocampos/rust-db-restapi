CREATE TABLE client (
  id         SERIAL PRIMARY KEY,
  name       TEXT NOT NULL,
  password   TEXT NOT NULL,
  profession TEXT
);

INSERT INTO CLIENT (
  name,
  password,
  profession
) VALUES (
  'mahesh',
  'password1',
  'teacher'
), (
  'suresh',
  'password2',
  'librarian'
), (
  'ramesh',
  'password3',
  'clerk'
);