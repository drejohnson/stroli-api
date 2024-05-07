CREATE MIGRATION m1zzejp3w2ba6kznyh2xxyaxzhezv6nmru7ewoajebzp3cjq52cr2a
    ONTO m1mqc2wvg7wqdd3vt3u27chh3ui6exjupjmmts7k7xv5vwnbd7d5fq
{
  ALTER TYPE default::User {
      ALTER PROPERTY userRole {
          RENAME TO user_role;
      };
  };
  ALTER TYPE default::Item {
      ALTER PROPERTY created {
          RENAME TO created_at;
      };
  };
  ALTER TYPE default::Item {
      ALTER PROPERTY updated {
          RENAME TO updated_at;
      };
  };
  ALTER TYPE default::User {
      ALTER PROPERTY created {
          RENAME TO created_at;
      };
  };
  ALTER TYPE default::User {
      ALTER PROPERTY profileImage {
          RENAME TO profile_image;
      };
  };
  ALTER TYPE default::User {
      ALTER PROPERTY updated {
          RENAME TO updated_at;
      };
  };
};
