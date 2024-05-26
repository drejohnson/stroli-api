DEFINE TABLE post SCHEMALESS
    PERMISSIONS
        FOR select
            WHERE published = true
            OR user = $auth.id
            OR $auth.role = roles:admin
        FOR create
            WHERE user = $auth.id
        FOR update, delete
            WHERE user = $auth.id
            OR $auth.role = roles:admin
;

DEFINE FIELD title ON post TYPE string;
DEFINE FIELD description ON post TYPE string;
DEFINE FIELD content ON post TYPE string;
DEFINE FIELD published ON post TYPE bool DEFAULT false;
DEFINE FIELD created_at ON post TYPE datetime DEFAULT time::now() READONLY;
DEFINE FIELD updated_at ON post TYPE datetime DEFAULT time::now() VALUE time::now() READONLY;
DEFINE FIELD profile ON post TYPE record(profile) DEFAULT $auth.id;