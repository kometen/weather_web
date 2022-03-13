# weather_web
Receive weather-reports in json-format.

Get weather data from Statens Vegvesen as a JSON-array and push that to an database.

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
    longitude numeric(10,6)
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
