-- Get latest readings
select measurement_time_default, id, name, latitude, longitude,
  value->>'field_description' as field_description,
  value->>'measurement' as measurement
from (select  measurement_time_default, id, name, latitude, longitude, data from location_readings) location_readings, jsonb_array_elements(location_readings.data);

-- Move measurements to a json-array
select measurement_time_default, id, name, latitude, longitude,
  json_agg(
    jsonb_build_object(
      value->>'field_description',
      value->>'measurement'
    )
  )
from (select  measurement_time_default, id, name, latitude, longitude, data from location_readings) location_readings, jsonb_array_elements(location_readings.data)
group by measurement_time_default, id, name, latitude, longitude;

