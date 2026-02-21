CREATE DATABASE `mestockydb`;

CREATE TABLE `tb_category` (
  `id` bigint unsigned NOT NULL AUTO_INCREMENT,
  `name` varchar(100) NOT NULL,
  PRIMARY KEY (`id`),
  UNIQUE KEY `tb_category_unique` (`name`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci;

CREATE TABLE `tb_product` (
  `id` bigint unsigned NOT NULL AUTO_INCREMENT,
  `name` varchar(100) NOT NULL,
  `quantity` bigint unsigned NOT NULL,
  `min_quantity` bigint unsigned NOT NULL,
  `category_id` bigint unsigned NOT NULL,
  `description` varchar(5000) DEFAULT NULL,
  PRIMARY KEY (`id`),
  UNIQUE KEY `tb_product_unique` (`name`),
  KEY `tb_product_tb_category_FK` (`category_id`),
  CONSTRAINT `tb_product_tb_category_FK` FOREIGN KEY (`category_id`) REFERENCES `tb_category` (`id`) ON DELETE RESTRICT ON UPDATE RESTRICT
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci;

CREATE TABLE `tb_reason` (
  `id` bigint unsigned NOT NULL AUTO_INCREMENT,
  `name` varchar(100) NOT NULL,
  PRIMARY KEY (`id`),
  UNIQUE KEY `tb_return_reason_unique_name` (`name`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci;

CREATE TABLE `tb_report` (
  `id` bigint unsigned NOT NULL AUTO_INCREMENT,
  `date` datetime NOT NULL,
  `quantity` bigint unsigned NOT NULL,
  `reason_id` bigint unsigned NOT NULL,
  `product_id` bigint unsigned NOT NULL,
  `change_type` tinyint(1) NOT NULL,
  PRIMARY KEY (`id`),
  KEY `tb_report_tb_reason_FK` (`reason_id`),
  KEY `tb_report_tb_product_FK` (`product_id`),
  CONSTRAINT `tb_report_tb_product_FK` FOREIGN KEY (`product_id`) REFERENCES `tb_product` (`id`) ON DELETE CASCADE ON UPDATE RESTRICT,
  CONSTRAINT `tb_report_tb_reason_FK` FOREIGN KEY (`reason_id`) REFERENCES `tb_reason` (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci;

CREATE TABLE `tb_user` (
  `id` bigint unsigned NOT NULL AUTO_INCREMENT,
  `username` varchar(100) NOT NULL,
  `password` varchar(255) NOT NULL,
  `email` varchar(100) NOT NULL,
  `role` tinyint unsigned NOT NULL,
  `phone` char(15) DEFAULT NULL,
  PRIMARY KEY (`id`),
  UNIQUE KEY `tb_user_unique_email` (`email`),
  KEY `tb_user_tb_role_FK` (`role`)
) ENGINE=InnoDB AUTO_INCREMENT=2 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci;