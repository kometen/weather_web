# weather_web
Receive weather-reports in json-format.

Get weather data from Statens Vegvesen as a JSON-array and push that to a database.

Table-definition:

```
-- weather data
create table readings (
    measurement_time_default timestamp with time zone not null,
    id int not null,
    data Jsonb not null
);

create unique index measurement_time_default_id_unique_index on readings (measurement_time_default, id);

create table locations (
    publication_time timestamp with time zone not null,
    id int not null,
    name text not null,
    latitude numeric(10,6),
    longitude numeric(10,6),
    geom geometry(Point, 4326)
);

create unique index id_unique_index on locations (id);

-- find newest date and get the sites that have reported at that moment

create view latest_readings as
  select * from readings where measurement_time_default =
    (select measurement_time_default from readings order by measurement_time_default desc limit 1)
  order by id;

create view location_readings as
  select
    lr.measurement_time_default, lr.id, lc.name, lc.latitude, lc.longitude, lr.data
  from
    latest_readings lr join locations lc on lr.id = lc.id;


-- get readings from a particular location

create or replace function measurements_single_location_function(id integer, rows integer)
  returns table (id integer, name text, latitude text, longitude text, measurement_time_default timestamp with time zone, measurements jsonb)
as
$body$

select id, name, latitude::text, longitude::text, measurement_time_default,
  json_agg(
    jsonb_build_object(
      value->>'field_description',
      value->>'measurement'
    )
  ) as measurements
from (
      select
        measurement_time_default, l.id, name, latitude, longitude, data from readings r
        join locations l
        on l.id = r.id
        where l.id = $1
        order by measurement_time_default desc limit $2
    ) location_readings, jsonb_array_elements(location_readings.data)
group by measurement_time_default, id, name, latitude, longitude
order by measurement_time_default desc, id

$body$
language sql;

```

```
-- Add postgis to db. On FreeBSD this must be compiled manually since the packaged version depends on PostgreSQL 13.
CREATE EXTENSION postgis;

-- Alter existing table locations.
alter table locations add column geom geometry(Point, 4326);

-- Update geom column with data.
update locations set geom = ST_SetSRID(ST_MakePoint(longitude, latitude), 4326);

-- Add trigger that calculates the position.
-- https://stackoverflow.com/questions/16737738/postgresql-set-a-default-cell-value-according-to-another-cell-value
create or replace function trg_geom_function()
  returns trigger
  language plpgsql as
$func$
begin
  new.geom := ST_SetSRID(ST_MakePoint(new.longitude, new.latitude), 4326);
  return new;
end
$func$;

create trigger geom_default
  before insert on locations
  for each row
  when (new.geom is null and new.latitude is not null and new.longitude is not null)
  execute procedure trg_geom_function();

```

Get latest readings for locations closest to a latitude/longitude point.

```
create or replace function measurements_closest_locations_function(latitude numeric, longitude numeric, locations integer)
  returns table (id integer, name text, distance text, latitude text, longitude text, latest_reading timestamp with time zone, measurements jsonb)
as
$body$


select id, name, distance, latitude, longitude, latest_reading,
  json_agg(
    jsonb_build_object(
      value->>'field_description',
      value->>'measurement'
    )
  ) as measurements
from (

  select
    distinct on (r.id)
    r.measurement_time_default as latest_reading,
    r.id,
    r.data,
    l.name,
    l.latitude,
    l.longitude,
    l.distance
  from
    readings r
  join
  (

    select
      id,
      name,
      latitude,
      longitude,
      locations.geom <-> (select ST_AsEWKT(ST_SetSRID(ST_MakePoint($2, $1), 4326), 1))::geometry AS distance
    from
      locations
    order by
      distance
    limit $3

  ) l

on r.id = l.id
order by
  r.id, r.measurement_time_default desc
limit $3

) parsed, jsonb_array_elements(data) -- <- jsonb_array_elements(data) unwraps the json-structure and it can be used in jsonb_build_object()
group by  id, name, latitude, longitude, distance, latest_reading
order by distance

$body$
language sql;

$body$
language sql;

```
