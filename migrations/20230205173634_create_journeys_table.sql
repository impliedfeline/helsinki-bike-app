CREATE TABLE journeys(
  id bigserial NOT NULL,
  PRIMARY KEY (id),
  departure_time timestamp NOT NULL,
  return_time timestamp NOT NULL,
  departure_station_id text NOT NULL,
  return_station_id text NOT NULL,
  distance_m real NOT NULL,
  duration_sec real NOT NULL
);

ALTER TABLE journeys
  ADD CONSTRAINT u_stats UNIQUE (departure_time, return_time, departure_station_id, return_station_id);
