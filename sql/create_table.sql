CREATE TABLE images (
    id          BIGSERIAL   NOT NULL PRIMARY KEY,
    file_path   text        NOT NULL,
    file_name   text        NOT NULL,
    digitized_at TIMESTAMP,
    props       jsonb,
    created_at  TIMESTAMP   NOT NULL default CURRENT_TIMESTAMP,
    updated_at  TIMESTAMP   NOT NULL default CURRENT_TIMESTAMP,
    UNIQUE(file_path, file_name)
);




rep_time           | timestamp without time zone | not null
date_time_original | text                        |
date_create        | text                        |
modified_date      | text                        |
