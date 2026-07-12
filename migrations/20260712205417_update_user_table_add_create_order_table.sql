-- Add migration script here
CREATE TABLE
    assets (
        id UUID PRIMARY KEY DEFAULT gen_random_uuid (),
        name TEXT NOT NULL,
        symbol TEXT NOT NULL UNIQUE,
        img TEXT NOT NULL,
        created_at TIMESTAMPTZ NOT NULL DEFAULT now (),
        updated_at TIMESTAMPTZ NOT NULL DEFAULT now ()
    );

CREATE TABLE
    balances (
        id UUID PRIMARY KEY DEFAULT gen_random_uuid (),
        user_id UUID NOT NULL REFERENCES users (id),
        asset TEXT NOT NULL,
        available TEXT NOT NULL,
        locked TEXT NOT NULL,
        created_at TIMESTAMPTZ NOT NULL DEFAULT now (),
        updated_at TIMESTAMPTZ NOT NULL DEFAULT now (),
        UNIQUE (user_id, asset)
    );

CREATE TYPE side AS ENUM ('buy', 'sell');

CREATE TYPE type AS ENUM ('limit', 'market');

CREATE TYPE order_status AS ENUM ('open', 'partially_filled', 'filled', 'cancelled');

CREATE TABLE
    orders (
        id UUID PRIMARY KEY DEFAULT gen_random_uuid (),
        quantity TEXT NOT NULL,
        price TEXT NOT NULL,
        side side NOT NULL,
        type type NOT NULL,
        status order_status NOT NULL DEFAULT 'open',
        user_id UUID NOT NULL REFERENCES users (id),
        market TEXT NOT NULL,
        created_at TIMESTAMPTZ NOT NULL DEFAULT now (),
        updated_at TIMESTAMPTZ NOT NULL DEFAULT now ()
    );

CREATE TABLE
    fills (
        id UUID PRIMARY KEY DEFAULT gen_random_uuid (),
        maker_user_id UUID NOT NULL REFERENCES users (id),
        taker_user_id UUID NOT NULL REFERENCES users (id),
        maker_order_id UUID NOT NULL REFERENCES orders (id),
        taker_order_id UUID NOT NULL REFERENCES orders (id),
        price TEXT NOT NULL,
        quantity TEXT NOT NULL,
        market TEXT NOT NULL,
        created_at TIMESTAMPTZ NOT NULL DEFAULT now (),
        updated_at TIMESTAMPTZ NOT NULL DEFAULT now ()
    );

CREATE TABLE
    candles (
        id UUID PRIMARY KEY DEFAULT gen_random_uuid (),
        market TEXT NOT NULL,
        open TEXT NOT NULL,
        high TEXT NOT NULL,
        low TEXT NOT NULL,
        close TEXT NOT NULL,
        timestamp TEXT NOT NULL,
        created_at TIMESTAMPTZ NOT NULL DEFAULT now ()
    );