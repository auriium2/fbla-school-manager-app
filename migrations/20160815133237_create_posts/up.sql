CREATE TABLE students (
  id INT PRIMARY KEY,
  name VARCHAR NOT NULL,
  grade INT NOT NULL
);

CREATE TABLE events (
  id INT PRIMARY KEY,
  name VARCHAR NOT NULL,
  point_value INT NOT NULL
);

CREATE TABLE events_participation (
  student_id INT NOT NULL,
  event_id INT NOT NULL,
  FOREIGN KEY (student_id) REFERENCES students(student_id),
  FOREIGN KEY (event_id) REFERENCES events(event_id)
);
