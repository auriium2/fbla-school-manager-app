CREATE TABLE students (
  student_id INT PRIMARY KEY NOT NULL DEFAULT AUTOINCREMENT,
  name VARCHAR NOT NULL,
  grade INT NOT NULL
);

CREATE TABLE events (
  event_id INT PRIMARY KEY NOT NULL DEFAULT AUTOINCREMENT,
  name VARCHAR NOT NULL,
  point_value INT NOT NULL
);

CREATE TABLE events_participaton (
  record_id PRIMARY KEY NOT NULL,
  student_id INT NOT NULL,
  event_id INT NOT NULL,
  FOREIGN KEY (student_id) REFERENCES students(student_id),
  FOREIGN KEY (event_id) REFERENCES events(event_id)
);

INSERT INTO events VALUES (0,"default",0);


CREATE VIEW students_with_points AS
SELECT students.student_id, students.name, students.grade, SUM(events.point_value)
       FROM events
       JOIN events_participaton ON events_participaton.event_id = events.event_id
       JOIN students ON students.student_id = events_participaton.student_id;