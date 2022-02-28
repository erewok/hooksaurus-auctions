-- We go in reverse order here.
drop table auction_item_delivery;

drop index article_tags_gin;
drop table article;

drop index auction_item_bid_auction_item_ids;
drop table auction_item_bid;

drop index auction_item_tags_gin;
drop index auction_item_auction_ids;
drop index auction_item_organization_ids;
drop table auction_item;

drop table auction;
drop table organization;
drop table "user";

drop table address;
