-- User scope
DEFINE SCOPE user SESSION 1d SIGNUP (CREATE user CONTENT {
	email: $email,
	name: $name,
	password: crypto::argon2::generate($password)
}) SIGNIN (SELECT * FROM user WHERE email = $email AND crypto::argon2::compare(password, $password));

-- User schema
DEFINE TABLE user SCHEMAFULL
	PERMISSIONS
		FOR select, update, delete WHERE id = $auth.id;

DEFINE FIELD name ON user TYPE string;
DEFINE FIELD email ON user TYPE string ASSERT string::is::email($value);
DEFINE FIELD password ON user TYPE string;

DEFINE FIELD name ON user TYPE string;

-- Post schema
DEFINE TABLE profile SCHEMALESS;
DEFINE FIELD username ON profile TYPE string UNIQUE;
DEFINE FIELD user ON profile TYPE record(Users);

-- Create relation table
DEFINE TABLE create TYPE RELATION;

-- Define a follow RELATION table, with constraints on the type of relation which can be stored
DEFINE TABLE following TYPE RELATION FROM profile TO profile;

-- Just a RELATION table, no constraints on the type of table
DEFINE TABLE likes TYPE RELATION;

-- Define a relation table, and constraint the type of relation which can be stored
-- DEFINE TABLE likes TYPE RELATION FROM profile TO video_post;

-- Define cleanup event to be triggered when a user is deleted
DEFINE EVENT user_deleted
	ON user
	WHEN $event = "DELETE"
	THEN {
		// TODO: Delete all posts, comments, edge relationship, etc. associated with the user
	}
