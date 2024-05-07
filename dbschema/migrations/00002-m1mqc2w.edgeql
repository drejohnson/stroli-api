CREATE MIGRATION m1mqc2wvg7wqdd3vt3u27chh3ui6exjupjmmts7k7xv5vwnbd7d5fq
    ONTO m1n3migjl5dm3ghpbn7kou5as7obqvd3xeflfn524lx5jgh6ujbxnq
{
  ALTER TYPE default::User {
      CREATE REQUIRED PROPERTY username: std::str {
          SET REQUIRED USING (<std::str>{});
      };
      CREATE CONSTRAINT std::exclusive ON (std::str_trim(std::str_lower(.username)));
      ALTER PROPERTY email {
          CREATE CONSTRAINT std::exclusive;
          SET REQUIRED USING (<std::str>{});
      };
      CREATE PROPERTY email_verified: std::bool {
          SET default := false;
      };
      CREATE PROPERTY profileImage: std::str;
  };
};
