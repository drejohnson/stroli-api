DEFINE roles:admin SET name = 'Admin'
DEFINE roles:moderator SET name = 'Moderator'
DEFINE roles:creator SET name = 'Creator'
DEFINE roles:user SET name = 'User'

DEFINE TABLE user SCHEMAFULL
  PERMISSIONS FOR select, update, delete, create
  WHERE
    $scope = "user"
    AND $token.iss = "https://api.workos.com/"
    AND email = $token.email
    AND $token.email_verified = true
DEFINE FIELD email ON user TYPE string ASSERT string::is::email($value) UNIQUE; 
DEFINE FIELD role ON user TYPE record(roles) DEFAULT roles:user;
DEFINE FIELD created ON user 
    VALUE $before OR time::now()
    DEFAULT time::now();
DEFINE FIELD updated ON user 
    VALUE time::now()
    DEFAULT time::now();

CREATE TABLE account SCHEMALESS;
DEFINE FIELD user_id ON account TYPE link <user> UNIQUE;  
DEFINE FIELD first_name ON account TYPE string ASSERT string::length($value) >= 2;
DEFINE FIELD last_name ON account TYPE string ASSERT string::length($value) >= 2; 
DEFINE FIELD username ON account TYPE string ASSERT string::length($value) >= 3 UNIQUE;
DEFINE FIELD birthdate ON account TYPE datetime ASSERT $value < time::now() PERMISSIONS FOR update NONE;
DEFINE FIELD profile_picture ON TABLE user TYPE option<string>
  DEFAULT None;
DEFINE FIELD bio ON account TYPE option<string>
  DEFAULT None;
DEFINE FIELD status ON account TYPE string
  DEFAULT "Inactive"
-- Allow only these values in the array
  ASSERT $value INSIDE ["Active", "Inactive", "Suspended", "Deleted"];
