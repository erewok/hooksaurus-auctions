SET timezone = 'utc';

-- This extension gives us `uuid_generate_v1mc()` which generates UUIDs that cluster better than `gen_random_uuid()`
-- while still being difficult to predict and enumerate.
-- Also, while unlikely, `gen_random_uuid()` can in theory produce collisions which can trigger spurious errors on
-- insertion, whereas it's much less likely with `uuid_generate_v1mc()`.
create extension if not exists "uuid-ossp";

-- While `created_at` can just be `default now()`, setting `updated_at` on update requires a trigger which
-- is a lot of boilerplate. These two functions save us from writing that every time as instead we can just do
--
-- select trigger_updated_at('<table name>');
--
-- after a `CREATE TABLE`.
create or replace function set_updated_at()
    returns trigger as
$$
begin
    NEW.updated_at = now();
    return NEW;
end;
$$ language plpgsql;

create or replace function trigger_updated_at(tablename regclass)
    returns void as
$$
begin
    execute format('CREATE TRIGGER set_updated_at
        BEFORE UPDATE
        ON %s
        FOR EACH ROW
        WHEN (OLD is distinct from NEW)
    EXECUTE FUNCTION set_updated_at();', tablename);
end;
$$ language plpgsql;


-- etag trigger to automatically set a new one on each update
-- this makes a lot easier for tables to report on each object
-- whether it has changed for the user or not.
--
-- after a `CREATE TABLE`.
create or replace function set_etag()
    returns trigger as
$$
begin
    NEW.etag = uuid_generate_v1mc();
    return NEW;
end;
$$ language plpgsql;

create or replace function trigger_etag(tablename regclass)
    returns void as
$$
begin
    execute format('CREATE TRIGGER set_etag
        BEFORE UPDATE
        ON %s
        FOR EACH ROW
        WHEN (OLD is distinct from NEW)
    EXECUTE FUNCTION set_etag();', tablename);
end;
$$ language plpgsql;

-- Finally, this is a text collation that sorts text case-insensitively, useful for `UNIQUE` indexes
-- over things like usernames and emails, without needing to remember to do case-conversion.
create collation  if not exists case_insensitive (provider = icu, locale = 'und-u-ks-level2', deterministic = false);

-- COMMON FIELDS BELOW --
-- - created:  timestamptz when object created at
-- - modified: timestamptz when object last modified at
-- - etag:     identifier that signals whether object has been modified


-- ADDRESS TABLE --
create table address (
    address_id            uuid primary key  default uuid_generate_v1mc(),
    street_address1       text not null,
    street_address2       text,
    street_address3       text,
    city                  text not null,
    state_province_county text not null,
    postal_code           text,
    country_code          text,
    latitude              double precision,
    longitude             double precision,

    created_at      timestamptz not null default now(),
    updated_at      timestamptz,
    etag            uuid not null
);
SELECT trigger_updated_at('address');
select trigger_etag('address');

-- ORGANIZATION TABLE --
-- Organizations are businesses or organizations that
-- have donated auction items, and in the case of non-profits
-- can receive proceeds from an auction
create table organization
(
    organization_id  uuid primary key     default uuid_generate_v1mc(),
    org_type      text                    not null default 'business',
    name          text                    not null default '',
    description   text,
    image         text,
    email         text collate "case_insensitive" not null,
    website       text collate "case_insensitive" not null,
    contact_name  text,
    phone_number  text,
    alt_phone_number text,
    -- need address
    primary_address_id uuid not null references "address" (address_id) on delete set null,
    -- defaults
    created_at    timestamptz not null default now(),
    updated_at    timestamptz not null default now(),
    etag          uuid not null
);
SELECT trigger_updated_at('organization');
select trigger_etag('organization');

-- USER TABLE --
-- users can place bids, browse auctions, or modify their
create table "user"
(
    user_id       uuid primary key                                default uuid_generate_v1mc(),
    -- By applying our custom collation we can simply mark this column as `unique` and Postgres will enforce
    -- case-insensitive uniqueness for us, and lookups over `email` will be case-insensitive by default.
    --
    -- Note that this collation doesn't support the `LIKE`/`ILIKE` operators so if you want to do searches
    -- over `email` you will want a separate index with the default collation:
    --
    -- create index on "user" (email collate "ucs_basic");
    --
    -- select * from "user" where (email collate "ucs_basic") ilike ($1 || '%')
    email         text collate "case_insensitive" unique not null,
    password_hash text                                   not null,
    bio           text                                   not null default '',
    image         text,
    first_name    text,
    last_name     text,
    phone_number  text,
    alt_phone_number text,
    role          text default 'member',
    -- need address to populate for shipping
    address_id uuid not null references "address" (address_id) on delete set null,
    -- defaults
    created_at    timestamptz not null default now(),
    updated_at    timestamptz not null default now(),
    etag          uuid not null
);

SELECT trigger_updated_at('"user"');
select trigger_etag('"user"');

-- AUCTION TABLE --
-- An auction is a collection of auctions with a start and end date 10 days from now (by default)
create table auction
(
    auction_id  uuid primary key     default uuid_generate_v1mc(),
    title       text        not null,
    description text        not null default '',
    start_date  timestamptz not null default now(),
    end_date    timestamptz not null default now() + interval '10' day,
    benefits_organization_id uuid references organization (organization_id) on delete set null,
    -- defaults
    created_at  timestamptz not null default now(),
    updated_at  timestamptz not null default now(),
    etag        uuid not null
);
SELECT trigger_updated_at('auction');
select trigger_etag('auction');

-- AUCTION ITEM TABLE --
create table auction_item
(
    auction_item_id  uuid primary key     default uuid_generate_v1mc(),
    -- Each item must be a member of an auction
    auction_id       uuid not null references "auction" (auction_id) on delete cascade,
    -- Some items are part of "baskets" of items: a "basket" is an auction_item that these roll up to
    basket_id        uuid references "auction_item" (auction_item_id) on delete cascade,
    -- Expected retail value
    expected_retail_value  decimal(15, 6) not null default '1.0',
    -- defaults to zero, but can be set higher. Could also call this a "target bid"?
    minimum_bid_amount  decimal(15, 6) not null default '0.0',
    -- some things may be immediately winnable by bid for a fixed price.
    -- this is the "buy it now" price, nullable.
    buy_it_now_amount  decimal(15, 6),

    -- title, description, tags (for searching), photos
    title       text        not null,
    description text        not null default '',
    featured_image_filepath text not null,
    image_dir   text        not null,
    tag_list    text[]      not null,
    -- Links to organizations: an org may have donated this item
    donated_by_organization_id     uuid references organization (organization_id) on delete set null,
    -- If the proceeds from this item go to a named organization, that may be linked here
    benefits_organization_id       uuid references organization (organization_id) on delete set null,
    -- this item can potentially only be bid on while active; We can set the active start/end date here.
    active_start_date  timestamptz not null default now(),
    active_end_date    timestamptz   not null,

    -- defaults
    created_at  timestamptz not null default now(),
    updated_at  timestamptz not null default now(),
    etag        uuid not null
);

select trigger_updated_at('auction_item');
select trigger_etag('auction_item');

-- This should speed up searching with tags.
create index auction_item_tags_gin on auction_item using gin (tag_list);
-- This should speed up searching for all items in an auction.
create index auction_item_auction_ids on auction_item using btree (auction_id);
-- This should speed up searching for all auction_items benefitting a particular org.
create index auction_item_organization_ids on auction_item using btree (benefits_organization_id);

-- AUCTION ITEM BID TABLE --
-- This table represents bids placed on auction_items
create table auction_item_bid
(
    auction_item_bid_id uuid    primary key  default uuid_generate_v1mc(),
    -- Each bid can be placed on a single item
    auction_item_id     uuid        not null references auction_item (auction_item_id) on delete cascade,
    -- Do not enforce uniqueness like with each bid per user like via a composite primary key.
    -- Users can submit multiple bids for the same item, in other words.
    user_id             uuid        not null references "user" (user_id) on delete cascade,
    amount              decimal(15, 6)      not null default '0.0',
    -- if someone sets a MAX bid amount, we can automatically increase their bid in relation to other bids
    max_bid_amount      decimal(15, 6),
    -- The winning bid is not necesarily the largest: someone may back out after bidding.
    -- This flag will likely get set by an admin at auction or item-bidding close.
    is_winning_bid      boolean     not null default false,
    created_at  timestamptz not null default now(),
    updated_at  timestamptz not null default now(),
    etag        uuid not null
);

select trigger_updated_at('auction_item_bid');
select trigger_etag('auction_item_bid');

-- This should speed up searching for all bids for an item (and probably all bids for an auction)
create index auction_item_bid_auction_item_ids on auction_item_bid using btree (auction_item_id);

-- SHIPPING of ITEMS IS HANDLED HERE --
create table auction_item_delivery
(
    auction_item_bid_id uuid not null references "auction_item_bid" (auction_item_bid_id) on delete cascade,
    user_id            uuid not null references "auction_item" (auction_item_id) on delete cascade,
    shipping_address   uuid not null references "address" (address_id) on delete cascade,
    shipping_fee       decimal(15, 6) default '0.0',
    shipped_datetime   timestamptz,
    delivered          timestamptz,
    shipping_exception text,
    sms_updates_number text,
    email_contact      text,
    signature_name     text,
    signed_for_by      text,
    carrier            text,
    tracking_number    text,
    -- defaults
    created_at      timestamptz not null default now(),
    updated_at      timestamptz not null default now(),
    etag            uuid not null,
    -- Enforce uniqueness like.
    primary key (auction_item_bid_id, user_id)
);
select trigger_updated_at('auction_item_delivery');
select trigger_etag('auction_item_delivery');


-- AUCTION ARTICLES --
-- for  news / posts / updates
create table article
(
    article_id  uuid primary key     default uuid_generate_v1mc(),
    -- An article _may_ be attached to an ongoing auction
    auction_id  uuid                 references "auction" (auction_id) on delete cascade,
    user_id     uuid        not null references "user" (user_id) on delete cascade,
    slug        text unique not null,
    title       text        not null,
    description text        not null,
    body        text        not null,
    tag_list    text[]      not null,
    featured_image_filepath text not null,
    created_at  timestamptz not null default now(),
    updated_at  timestamptz not null default now(),
    etag        uuid not null
);

select trigger_updated_at('article');
select trigger_etag('article');

-- This should speed up searching with tags.
create index article_tags_gin on article using gin (tag_list);
