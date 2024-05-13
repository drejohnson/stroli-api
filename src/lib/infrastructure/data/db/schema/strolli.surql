-- ------------------------------
-- OPTION
-- ------------------------------

OPTION IMPORT;

-- ------------------------------
-- USERS
-- ------------------------------

-- DEFINE USER admin ON ROOT PASSHASH '$argon2id$v=19$m=19456,t=2,p=1$esMYHgZig+Mayv7AtthPTg$pafTBRYnQEE81fo/tl2jk5LWfbyOlChTmmMEcIGSjp0' ROLES OWNER;

-- ------------------------------
-- FUNCTIONS
-- ------------------------------

DEFINE FUNCTION fn::number_of_unpublished_content() { RETURN (SELECT count() FROM content WHERE content_status NOTINSIDE ['published', 'moderation'] GROUP ALL); } PERMISSIONS FULL;

-- ------------------------------
-- TABLE: creator
-- ------------------------------

DEFINE TABLE creator SCHEMALESS PERMISSIONS NONE;

-- ------------------------------
-- TABLE: content_likes
-- ------------------------------

DEFINE TABLE content_likes SCHEMALESS AS SELECT count() AS number_of_likes, content.title AS content FROM like GROUP BY content;

-- ------------------------------
-- TABLE: daily_followers
-- ------------------------------

DEFINE TABLE daily_followers SCHEMALESS AS SELECT count() AS number_of_followers, date_trunc('day', created_at) AS day FROM follow GROUP BY day;

-- ------------------------------
-- TABLE: follow
-- ------------------------------

DEFINE TABLE follow SCHEMALESS PERMISSIONS NONE;

-- ------------------------------
-- TABLE: account
-- ------------------------------

DEFINE TABLE account SCHEMALESS PERMISSIONS NONE;

-- ------------------------------
-- TABLE: content
-- ------------------------------

DEFINE TABLE content SCHEMALESS PERMISSIONS NONE;

-- ------------------------------
-- TABLE: like
-- ------------------------------

DEFINE TABLE like SCHEMALESS PERMISSIONS NONE;

-- ------------------------------
-- TABLE: comment
-- ------------------------------

DEFINE TABLE comment SCHEMALESS PERMISSIONS NONE;
