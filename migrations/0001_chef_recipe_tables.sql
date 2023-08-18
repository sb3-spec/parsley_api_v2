-- Chef
CREATE TABLE IF NOT EXISTS chef(
    -- ctime timestamp with time zone DEFAULT now(),
    auth_id text primary key,
    username varchar(255),
    email varchar(255),
    custom_tags text[] DEFAULT array[]::text[]
);

-- Recipe
CREATE TABLE IF NOT EXISTS recipe(
    id bigserial primary key,
    cid text NOT NULL,
    ctime timestamp with time zone DEFAULT now(),
    mid text, -- modifier user id
    mtime timestamp with time zone,
    title text NOT NULL,
    header text DEFAULT '',
    ingredients text[] DEFAULT array[]::text[],
    steps text[] DEFAULT array[]::text[],
    tags text[] DEFAULT array[]::text[],
    image_url text DEFAULT '',
    cook_time text DEFAULT '',
    prep_time text DEFAULT '',
    total_time text DEFAULT '',

    CONSTRAINT fk_chef
        FOREIGN KEY(cid)
        REFERENCES chef(auth_id)
);