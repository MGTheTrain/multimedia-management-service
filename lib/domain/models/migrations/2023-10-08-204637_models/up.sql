CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- Create the 'subtitle_track' table
CREATE TABLE video_track (
  id UUID DEFAULT uuid_generate_v4 (),
  container_meta_id UUID NOT NULL,
  name VARCHAR NOT NULL,
  media_type VARCHAR NOT NULL,
  width INTEGER NOT NULL,
  height INTEGER NOT NULL,
  bit_rate INTEGER NOT NULL,
  frame_rate INTEGER NOT NULL,
  PRIMARY KEY (id)
);

-- Create the 'audio_track' table
CREATE TABLE audio_track (
  id UUID DEFAULT uuid_generate_v4 (),
  container_meta_id UUID NOT NULL,
  name VARCHAR NOT NULL,
  media_type VARCHAR NOT NULL,
  bit_rate INTEGER NOT NULL,
  channel_config VARCHAR NOT NULL,
  sample_frequenz INTEGER NOT NULL,
  PRIMARY KEY (id)
);

-- Create the 'subtitle_track' table
CREATE TABLE subtitle_track (
  id UUID DEFAULT uuid_generate_v4 (),
  container_meta_id UUID NOT NULL,
  name VARCHAR NOT NULL,
  media_type VARCHAR NOT NULL,
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
  video_track_id UUID NOT NULL,
  audio_track_id UUID NOT NULL,
  subtitle_track_id UUID NOT NULL,
  file_size_in_kb BIGINT NOT NULL,
  duration DOUBLE PRECISION NOT NULL,
  PRIMARY KEY (id)
);
