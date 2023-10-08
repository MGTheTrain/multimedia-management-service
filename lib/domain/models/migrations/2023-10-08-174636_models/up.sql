-- CREATE TYPE file_meta_type_mapping AS ENUM ('Invalid', 'Video', 'Audio', 'Subtitle');

-- file_type file_meta_type_mapping NOT NULL,
CREATE TABLE file_meta (
  id SERIAL PRIMARY KEY,
  name VARCHAR NOT NULL,
  file_type INTEGER NOT NULL,
  file_size_in_kb BIGINT NOT NULL
);

CREATE TABLE container_meta (
  id SERIAL PRIMARY KEY,
  date_time_created TIMESTAMP WITH TIME ZONE NOT NULL,
  date_time_updated TIMESTAMP WITH TIME ZONE NOT NULL,
  title VARCHAR NOT NULL,
  description VARCHAR NOT NULL,
  tags TEXT[] NOT NULL,
  file_meta_ids INTEGER[] NOT NULL
);