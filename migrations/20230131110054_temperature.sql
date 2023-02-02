-- Add migration script here
DROP DATABASE IF EXISTS `red_home`;

CREATE DATABASE `red_home` DEFAULT CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci;

SET NAMES utf8mb4;
SET FOREIGN_KEY_CHECKS = 0;
# set global time_zone = '+8:00';  ##修改mysql全局时区为北京时间，即我们所在的东8区
# set time_zone = '+8:00';  ##修改当前会话时区
# flush privileges;  #立即生效

USE `red_home`;

DROP TABLE IF EXISTS `temperature`;

CREATE TABLE `temperature`
(
    temperature DOUBLE NOT NULL,
    humidity    DOUBLE NOT NULL,
    created_at  Timestamp DEFAULT CURRENT_TIMESTAMP,
    updated_at  Timestamp DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP
);

DROP TABLE IF EXISTS `electricity`;

CREATE TABLE `electricity`
(
    remainder DOUBLE NOT NULL,
    unit    INT NOT NULL,
    created_at  Timestamp DEFAULT CURRENT_TIMESTAMP,
    updated_at  Timestamp DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP
);

DROP TABLE IF EXISTS `water`;

CREATE TABLE `water`
(
    remainder DOUBLE NOT NULL,
    unit    INT NOT NULL,
    created_at  Timestamp DEFAULT CURRENT_TIMESTAMP,
    updated_at  Timestamp DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP
);
