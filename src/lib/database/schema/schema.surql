DEFINE TABLE user SCHEMAFULL
  -- Authorized users can select, update, delete and create user records.
  -- Records that do not match the permissions will not be modified nor returned.
  PERMISSIONS FOR select, update, delete, create
  WHERE
    $scope = "user"
    AND $token.iss = "https://api.workos.com/"
    AND email = $token.email
    AND $token.email_verified = true
DEFINE FIELD email ON user TYPE string ASSERT string::is::email($value); 
DEFINE INDEX userEmailIndex ON TABLE user COLUMNS email UNIQUE;

-- Create schemaless profile table.
DEFINE TABLE profile SCHEMALESS;
DEFINE FIELD username ON profile TYPE string UNIQUE;
DEFINE FIELD user ON profile TYPE record(Users);

-- Specify access permissions for the 'post' table
DEFINE TABLE post SCHEMALESS
	PERMISSIONS
		FOR select
			-- Published posts can be selected
			WHERE published = true
			-- A user can select all their own posts
			OR user = $auth.id
		FOR create, update
			-- A user can create or update their own posts
			WHERE user = $auth.id
		FOR delete
			-- A user can delete their own posts
			WHERE user = $auth.id
			-- Or an admin can delete any posts
			OR $auth.admin = true
;

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
