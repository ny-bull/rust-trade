-- Your SQL goes here

CREATE TABLE trade_jobs
(
    id int AUTO_INCREMENT,
    bs tinyint NOT NULL,
    exchange varchar(10) NOT NULL,
    currency varchar(10) NOT NULL,
    order_id int,
    amount int NOT NULL,
    status BOOLEAN NOT NULL DEFAULT FALSE,
    create_at datetime NOT NULL,
    update_at datetime NOT NULL,
    trade_at datetime NOT NULL,
    PRIMARY KEY (id)
);