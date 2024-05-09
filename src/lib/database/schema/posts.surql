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

DEFINE FIELD title ON TABLE post TYPE string;
DEFINE FIELD description ON TABLE post TYPE string;
DEFINE FIELD content ON TABLE post TYPE string;
DEFINE FIELD published ON TABLE post TYPE bool DEFAULT false;
DEFINE FIELD created_at ON TABLE post TYPE datetime DEFAULT time::now();
DEFINE FIELD updated_at ON TABLE post TYPE datetime DEFAULT time::now() VALUE time::now();
DEFINE FIELD account ON TABLE post TYPE record(account) DEFAULT $auth.id;