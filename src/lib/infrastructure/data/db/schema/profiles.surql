DEFINE TABLE account SCHEMAFULL
    PERMISSIONS
        FOR select FULL
        FOR update, delete WHERE user = $auth.id OR $auth.role = roles:admin;

DEFINE FIELD username ON TABLE account TYPE string UNIQUE;
DEFINE FIELD user ON TABLE account TYPE record(Users);