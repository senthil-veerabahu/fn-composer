-- Table: public.users

-- DROP TABLE IF EXISTS public.users;

CREATE TABLE IF NOT EXISTS public.users
(
    id bigint NOT NULL DEFAULT nextval('users_id_seq'::regclass),
    created_at timestamp with time zone,
    updated_at timestamp with time zone,
    user_id uuid NOT NULL DEFAULT gen_random_uuid(),
    name text COLLATE pg_catalog."default" NOT NULL,
    first_name text COLLATE pg_catalog."default" NOT NULL,
    last_name text COLLATE pg_catalog."default" NOT NULL,
    email text COLLATE pg_catalog."default" NOT NULL,
    password text COLLATE pg_catalog."default" NOT NULL,
    phone_number text COLLATE pg_catalog."default",
    isemail_verfied boolean DEFAULT false,
    is_phone_verfied text COLLATE pg_catalog."default" DEFAULT 'Pending'::text,
    is_blocked boolean DEFAULT false,
    is_active boolean DEFAULT false,
    CONSTRAINT users_pkey PRIMARY KEY (id),
    CONSTRAINT users_email_key UNIQUE (email),
    CONSTRAINT users_phone_number_key UNIQUE (phone_number),
    CONSTRAINT users_user_id_key UNIQUE (user_id)
)

TABLESPACE pg_default;

ALTER TABLE IF EXISTS public.users
    OWNER to postgres;


-- Table: public.roles

-- DROP TABLE IF EXISTS public.roles;

CREATE TABLE IF NOT EXISTS public.roles
(
    id bigint NOT NULL DEFAULT nextval('roles_id_seq'::regclass),
    created_at timestamp with time zone,
    updated_at timestamp with time zone,
    name text COLLATE pg_catalog."default" NOT NULL,
    CONSTRAINT roles_pkey PRIMARY KEY (id)
)

TABLESPACE pg_default;

ALTER TABLE IF EXISTS public.roles
    OWNER to postgres;


-- Table: public.role_entities

-- DROP TABLE IF EXISTS public.role_entities;

CREATE TABLE IF NOT EXISTS public.role_entities
(
    id bigint NOT NULL DEFAULT nextval('role_entities_id_seq'::regclass),
    created_at timestamp with time zone,
    updated_at timestamp with time zone,
    entity_id text COLLATE pg_catalog."default",
    entity_name text COLLATE pg_catalog."default",
    role_id bigint,
    logger_id uuid,
    CONSTRAINT role_entities_pkey PRIMARY KEY (id),
    CONSTRAINT fk_role_entities_role FOREIGN KEY (role_id)
        REFERENCES public.roles (id) MATCH SIMPLE
        ON UPDATE NO ACTION
        ON DELETE NO ACTION,
    CONSTRAINT fk_users_role_entity FOREIGN KEY (logger_id)
        REFERENCES public.users (user_id) MATCH SIMPLE
        ON UPDATE NO ACTION
        ON DELETE NO ACTION
)

TABLESPACE pg_default;

ALTER TABLE IF EXISTS public.role_entities
    OWNER to postgres;


-- Table: public.products

-- DROP TABLE IF EXISTS public.products;

CREATE TABLE IF NOT EXISTS public.products
(
    id bigint NOT NULL DEFAULT nextval('products_id_seq'::regclass),
    created_at timestamp with time zone,
    updated_at timestamp with time zone,
    product_uuid uuid NOT NULL DEFAULT gen_random_uuid(),
    product_name text COLLATE pg_catalog."default" NOT NULL,
    category_id bigint,
    description text COLLATE pg_catalog."default" NOT NULL,
    family text COLLATE pg_catalog."default",
    variant text COLLATE pg_catalog."default" NOT NULL,
    discount bigint NOT NULL,
    CONSTRAINT products_pkey PRIMARY KEY (id),
    CONSTRAINT products_product_uuid_key UNIQUE (product_uuid),
    CONSTRAINT fk_categories_products FOREIGN KEY (category_id)
        REFERENCES public.categories (id) MATCH SIMPLE
        ON UPDATE NO ACTION
        ON DELETE NO ACTION
)

TABLESPACE pg_default;

ALTER TABLE IF EXISTS public.products
    OWNER to postgres;


-- Table: public.product_attributes

-- DROP TABLE IF EXISTS public.product_attributes;

CREATE TABLE IF NOT EXISTS public.product_attributes
(
    id bigint NOT NULL DEFAULT nextval('product_attributes_id_seq'::regclass),
    created_at timestamp with time zone,
    updated_at timestamp with time zone,
    product_attrb_uuid uuid NOT NULL DEFAULT gen_random_uuid(),
    product_id bigint,
    image_one text COLLATE pg_catalog."default",
    img_two text COLLATE pg_catalog."default",
    img_three text COLLATE pg_catalog."default",
    CONSTRAINT product_attributes_pkey PRIMARY KEY (id),
    CONSTRAINT product_attributes_product_attrb_uuid_key UNIQUE (product_attrb_uuid),
    CONSTRAINT fk_products_product_attribute FOREIGN KEY (product_id)
        REFERENCES public.products (id) MATCH SIMPLE
        ON UPDATE NO ACTION
        ON DELETE NO ACTION
)

TABLESPACE pg_default;

ALTER TABLE IF EXISTS public.product_attributes
    OWNER to postgres;

-- Table: public.permissions

-- DROP TABLE IF EXISTS public.permissions;

CREATE TABLE IF NOT EXISTS public.permissions
(
    id bigint NOT NULL DEFAULT nextval('permissions_id_seq'::regclass),
    created_at timestamp with time zone,
    updated_at timestamp with time zone,
    permission1 text COLLATE pg_catalog."default" NOT NULL,
    CONSTRAINT permissions_pkey PRIMARY KEY (id)
)

TABLESPACE pg_default;

ALTER TABLE IF EXISTS public.permissions
    OWNER to postgres;
    
-- Table: public.categories

-- DROP TABLE IF EXISTS public.categories;

CREATE TABLE IF NOT EXISTS public.categories
(
    id bigint NOT NULL DEFAULT nextval('categories_id_seq'::regclass),
    created_at timestamp with time zone,
    updated_at timestamp with time zone,
    category_uuid uuid NOT NULL DEFAULT gen_random_uuid(),
    catogery_name text COLLATE pg_catalog."default" NOT NULL,
    imgurl text COLLATE pg_catalog."default" NOT NULL,
    "order" bigint,
    CONSTRAINT categories_pkey PRIMARY KEY (id),
    CONSTRAINT categories_category_uuid_key UNIQUE (category_uuid)
)

TABLESPACE pg_default;

ALTER TABLE IF EXISTS public.categories
    OWNER to postgres;    