# weather_web
Receive weather-reports in json-format.

Get weather data from Statens Vegvesen as a JSON-array and push that to an database.

Table-definition:

```
create table readings (
    measurement_time_default timestamp with time zone not null,
    id int not null,
    index int not null,
    field_description text not null,
    measurement numeric(9,3)
);

create unique index measurement_time_default_id_index_unique_index on readings (measurement_time_default, id, index);

create table locations (
    publication_time timestamp with time zone not null,
    id int not null,
    name text not null,
    latitude numeric(10,6),
    longitude numeric(10,6)
);

create unique index id_unique_index on locations (id);
```