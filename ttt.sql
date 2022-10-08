-- public.email_verification definition

-- Drop table

-- DROP TABLE public.email_verification;

CREATE TABLE public.email_verification (
	id uuid NOT NULL,
	email varchar NOT NULL,
	time_generated timestamptz NOT NULL DEFAULT CURRENT_TIMESTAMP(0),
	expires_at timestamptz NOT NULL DEFAULT CURRENT_TIMESTAMP(0) + '01:00:00'::interval,
	CONSTRAINT email_verification_pk PRIMARY KEY (id),
	CONSTRAINT email_verification_unique UNIQUE (email)
);


-- public.games definition

-- Drop table

-- DROP TABLE public.games;

CREATE TABLE public.games (
	user1_id int8 NOT NULL,
	user2_id int8 NOT NULL,
	winner int8 NULL,
	end_time timestamptz NOT NULL DEFAULT CURRENT_TIMESTAMP(0),
	start_time timestamptz NOT NULL,
	game_id uuid NOT NULL,
	user1_elo int8 NOT NULL DEFAULT 0,
	user2_elo int8 NOT NULL DEFAULT 0,
	CONSTRAINT games_pk PRIMARY KEY (game_id)
);


-- public.user_stats definition

-- Drop table

-- DROP TABLE public.user_stats;

CREATE TABLE public.user_stats (
	user_id bigserial NOT NULL,
	wins int8 NOT NULL DEFAULT 0,
	losses int8 NOT NULL DEFAULT 0,
	draws int8 NOT NULL DEFAULT 0,
	elo int8 NOT NULL DEFAULT 1200,
	CONSTRAINT user_stats_pk PRIMARY KEY (user_id)
);


-- public.users definition

-- Drop table

-- DROP TABLE public.users;

CREATE TABLE public.users (
	username varchar NOT NULL,
	"password" varchar NOT NULL,
	email varchar NOT NULL,
	created_on timestamptz NOT NULL DEFAULT CURRENT_TIMESTAMP(0),
	is_admin bool NOT NULL DEFAULT false,
	email_verified bool NOT NULL DEFAULT false,
	is_banned bool NOT NULL DEFAULT false,
	ban_ends timestamptz NULL,
	user_id bigserial NOT NULL,
	guest bool NOT NULL DEFAULT false,
	CONSTRAINT users_pkey PRIMARY KEY (user_id),
	CONSTRAINT users_unique_email UNIQUE (email),
	CONSTRAINT users_unique_username UNIQUE (username)
);
