-- Your SQL goes here
CREATE TABLE IF NOT EXISTS "public"."extra_info" (
  "id_request" serial4,
  "cc1" char(16) NOT NULL,
  "cc2" char(16) NOT NULL,
  PRIMARY KEY ("id_request")
)
;