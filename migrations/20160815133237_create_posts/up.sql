CREATE TABLE students (
  student_id SERIAL PRIMARY KEY NOT NULL,
  name VARCHAR(100) NOT NULL,
  grade INT NOT NULL
);

CREATE TABLE school_events (
  event_id SERIAL PRIMARY KEY NOT NULL,
  name VARCHAR(100) NOT NULL,
  point_value INT NOT NULL
);

CREATE TABLE events_participation (
  record_id SERIAL PRIMARY KEY NOT NULL,
  student_id INT NOT NULL,
  event_id INT NOT NULL,
  FOREIGN KEY (student_id) REFERENCES students(student_id),
  FOREIGN KEY (event_id) REFERENCES school_events(event_id)
);

INSERT INTO school_events (name,point_value) VALUES ('name',0);


CREATE VIEW students_with_points AS
SELECT
  students.student_id,
  students.name,
  students.grade,
  COALESCE(SUM(school_events.point_value), 0) AS total_points
FROM
  students
LEFT JOIN
  events_participation ON students.student_id = events_participation.student_id
LEFT JOIN
  school_events ON events_participation.event_id = school_events.event_id
GROUP BY
  students.student_id;