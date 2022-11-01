-- Your SQL goes here

CREATE TABLE trade_jobs
(
    id int AUTO_INCREMENT,
    BS tinyint,
    type varchar(10),
    currency varchar(10),
    origin_id int,
    amount int,
    status tinyint,
    create_at datetime DEFAULT NULL,
    update_at datetime DEFAULT NULL,
    trade_at datetime DEFAULT NULL,
    PRIMARY KEY (id)
);