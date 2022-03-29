CREATE TABLE images (
    id          INTEGER     NOT NULL PRIMARY KEY AUTOINCREMENT,
    file_path   text        NOT NULL,
    file_name   text        NOT NULL,
    digitized_at TIMESTAMP,
    props       json,
    created_at  TIMESTAMP   NOT NULL default CURRENT_TIMESTAMP,
    updated_at  TIMESTAMP   NOT NULL default CURRENT_TIMESTAMP,
    UNIQUE(file_path, file_name)
);
