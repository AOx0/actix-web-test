-- This file should undo anything in `up.sql`
ALTER TABLE "public"."extra_info"
    ALTER COLUMN "id_request" SET DEFAULT nextval('extra_info_id_request_seq'::regclass),
    ALTER COLUMN "id_request" DROP IDENTITY;