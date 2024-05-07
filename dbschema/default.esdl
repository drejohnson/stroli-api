using extension auth;

module default {
  scalar type Role extending enum<admin, user, creator>;

  global current_user := (
    assert_single((
      select User
      filter .identity = global ext::auth::ClientTokenIdentity
    ))
  );

  type User {
    required identity: ext::auth::Identity {
      constraint exclusive;
    };
    required name: str;

    # unique email
    required email: str {
      constraint exclusive;
    };
    
    # unique username
    required username: str;
    constraint exclusive on (str_trim(str_lower(.username)));

    # profile picture
    profile_image: str;

    # email verified
    email_verified: bool {
      default := false;
    };

    # user role
    user_role: Role {
      default := "user";
    };

    created_at: datetime {
      rewrite insert using (datetime_of_statement());
    }
    updated_at: datetime {
      rewrite insert using (datetime_of_statement());
      rewrite update using (datetime_of_statement());
    }
  }

  type Item {
    required name: str;
    required created_by: User {
      default := global current_user;
    }

    created_at: datetime {
      rewrite insert using (datetime_of_statement());
    }
    updated_at: datetime {
      rewrite insert using (datetime_of_statement());
      rewrite update using (datetime_of_statement());
    }

    access policy admin_has_full_access
      allow all
      using (global current_user.user_role ?= Role.admin);
    access policy creator_has_full_access
      allow all
      using (.created_by ?= global current_user);
    access policy others_read_only
      allow select, insert;
  }
}