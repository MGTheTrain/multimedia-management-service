CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- Create the 'file_meta' table
CREATE TABLE file_meta (
  id UUID DEFAULT uuid_generate_v4 (),
  container_meta_id UUID NOT NULL,
  name VARCHAR NOT NULL,
  file_type INTEGER NOT NULL,
  PRIMARY KEY (id)
);

-- Create the 'container_meta' table
CREATE TABLE container_meta (
  id UUID DEFAULT uuid_generate_v4 (),
  date_time_created TIMESTAMP WITH TIME ZONE NOT NULL,
  date_time_updated TIMESTAMP WITH TIME ZONE NOT NULL,
  title VARCHAR NOT NULL,
  description VARCHAR NOT NULL,
  tags TEXT[] NOT NULL,
  file_meta_ids UUID[] NOT NULL,
  file_size_in_kb BIGINT NOT NULL,
  PRIMARY KEY (id)
);
