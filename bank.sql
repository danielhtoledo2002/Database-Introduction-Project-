SET NAMES utf8mb4;
SET FOREIGN_KEY_CHECKS = 0;

-- ----------------------------
-- Table structure for atms
-- ----------------------------
DROP TABLE IF EXISTS `atms`;
CREATE TABLE `atms` (
    `name` varchar(255) NOT NULL,
    `address` varchar(255) NOT NULL,
    `bank_id` int(10) unsigned NOT NULL,
    `money` double unsigned NOT NULL,
    PRIMARY KEY (`name`),
    KEY `bank_id_f` (`bank_id`),
    CONSTRAINT `bank_id_f` FOREIGN KEY (`bank_id`) REFERENCES `bancos` (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;

-- ----------------------------
-- Records of atms
-- ----------------------------
BEGIN;
INSERT INTO `atms` (`name`, `address`, `bank_id`, `money`) VALUES ('BB_021', 'Av. de los Insurgentes Sur 1323, Insurgentes Mixcoac, Benito Juárez, 03920 Ciudad de México, CDMX', 2, 123010);
INSERT INTO `atms` (`name`, `address`, `bank_id`, `money`) VALUES ('BB_156', 'Av. Revolución 1579, San Ángel, Álvaro Obregón, 01000 Ciudad de México, CDMX', 2, 220000);
INSERT INTO `atms` (`name`, `address`, `bank_id`, `money`) VALUES ('CB_102', 'Felipe Carrillo Puerto 3, Coyoacán, 04100 Ciudad de México, CDMX', 3, 190000);
INSERT INTO `atms` (`name`, `address`, `bank_id`, `money`) VALUES ('S_064', 'Oso 81, Col del Valle Centro, Benito Juárez, 03100 Ciudad de México, CDMX', 2, 200000);
INSERT INTO `atms` (`name`, `address`, `bank_id`, `money`) VALUES ('S_067', 'Av. Insurgentes Sur 1883, Guadalupe Inn, Álvaro Obregón, CDMX', 2, 152100);
COMMIT;

-- ----------------------------
-- Table structure for bancos
-- ----------------------------
DROP TABLE IF EXISTS `bancos`;
CREATE TABLE `bancos` (
    `id` int(10) unsigned NOT NULL AUTO_INCREMENT,
    `name` varchar(255) NOT NULL,
    PRIMARY KEY (`id`)
) ENGINE=InnoDB AUTO_INCREMENT=5 DEFAULT CHARSET=utf8mb4;

-- ----------------------------
-- Records of bancos
-- ----------------------------
BEGIN;
INSERT INTO `bancos` (`id`, `name`) VALUES (2, 'Santander');
INSERT INTO `bancos` (`id`, `name`) VALUES (3, 'BBVA');
INSERT INTO `bancos` (`id`, `name`) VALUES (4, 'Citibanamex');
COMMIT;

-- ----------------------------
-- Table structure for cards
-- ----------------------------
DROP TABLE IF EXISTS `cards`;
CREATE TABLE `cards` (
     `number` varchar(16) NOT NULL,
     `bank_id` int(10) unsigned NOT NULL,
     `cvv` int(10) unsigned NOT NULL,
     `nip` int(11) NOT NULL,
     `expiration_date` date NOT NULL,
     `balance` double unsigned NOT NULL,
     `type` varchar(10) NOT NULL,
     `expired` tinyint(1) NOT NULL,
     `try` int(10) unsigned NOT NULL DEFAULT 0,
     PRIMARY KEY (`number`),
     KEY `bank_id_fk` (`bank_id`),
     CONSTRAINT `bank_id_fk` FOREIGN KEY (`bank_id`) REFERENCES `bancos` (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;

-- ----------------------------
-- Records of cards
-- ----------------------------
BEGIN;
INSERT INTO `cards` (`number`, `bank_id`, `cvv`, `nip`, `expiration_date`, `balance`, `type`, `expired`, `try`) VALUES ('1426045760345700', 2, 423, 1233, '2025-11-11', 45215, 'Debit', 0, 0);
INSERT INTO `cards` (`number`, `bank_id`, `cvv`, `nip`, `expiration_date`, `balance`, `type`, `expired`, `try`) VALUES ('1426045781603457', 2, 423, 1233, '2025-11-11', 45215, 'Debit', 0, 0);
INSERT INTO `cards` (`number`, `bank_id`, `cvv`, `nip`, `expiration_date`, `balance`, `type`, `expired`, `try`) VALUES ('4578015464893204', 2, 130, 1233, '2025-06-23', 17590, 'Credit', 0, 0);
INSERT INTO `cards` (`number`, `bank_id`, `cvv`, `nip`, `expiration_date`, `balance`, `type`, `expired`, `try`) VALUES ('7598150468901754', 2, 457, 1233, '2025-12-05', 7546, 'Credit', 0, 0);
INSERT INTO `cards` (`number`, `bank_id`, `cvv`, `nip`, `expiration_date`, `balance`, `type`, `expired`, `try`) VALUES ('8105460479831056', 3, 684, 1233, '2023-03-12', 24321, 'Debit', 0, 0);
COMMIT;

-- ----------------------------
-- Table structure for customers
-- ----------------------------
DROP TABLE IF EXISTS `customers`;
CREATE TABLE `customers` (
     `id` int(11) unsigned NOT NULL AUTO_INCREMENT,
     `name` varchar(255) NOT NULL,
     `surname` varchar(255) NOT NULL,
     `card_number` varchar(16) NOT NULL,
     PRIMARY KEY (`id`),
     KEY `card_number_fk` (`card_number`),
     CONSTRAINT `card_number_fk` FOREIGN KEY (`card_number`) REFERENCES `cards` (`number`)
) ENGINE=InnoDB AUTO_INCREMENT=6 DEFAULT CHARSET=utf8mb4;

-- ----------------------------
-- Records of customers
-- ----------------------------
BEGIN;
INSERT INTO `customers` (`id`, `name`, `surname`, `card_number`) VALUES (1, 'Arnulfo', 'Carrera', '4578015464893204');
INSERT INTO `customers` (`id`, `name`, `surname`, `card_number`) VALUES (2, 'Ana', 'Armira', '7598150468901754');
INSERT INTO `customers` (`id`, `name`, `surname`, `card_number`) VALUES (3, 'María', 'Vásquez', '8105460479831056');
INSERT INTO `customers` (`id`, `name`, `surname`, `card_number`) VALUES (4, 'Edgar', 'Culajay', '1426045781603457');
INSERT INTO `customers` (`id`, `name`, `surname`, `card_number`) VALUES (5, 'Lilian', 'Rodríguez', '1426045760345700');
COMMIT;

-- ----------------------------
-- Table structure for deposits
-- ----------------------------
DROP TABLE IF EXISTS `deposits`;
CREATE TABLE `deposits` (
    `id` int(10) unsigned NOT NULL AUTO_INCREMENT,
    `amount` double unsigned NOT NULL,
    `date` date NOT NULL,
    `card_number` varchar(16) NOT NULL,
    `atm_name` varchar(255) NOT NULL,
    PRIMARY KEY (`id`),
    KEY `card_number_fk2` (`card_number`),
    KEY `atm_name_fk2` (`atm_name`),
    CONSTRAINT `atm_name_fk2` FOREIGN KEY (`atm_name`) REFERENCES `atms` (`name`),
    CONSTRAINT `card_number_fk2` FOREIGN KEY (`card_number`) REFERENCES `cards` (`number`)
) ENGINE=InnoDB AUTO_INCREMENT=8 DEFAULT CHARSET=utf8mb4;

-- ----------------------------
-- Records of deposits
-- ----------------------------
BEGIN;
INSERT INTO `deposits` (`id`, `amount`, `date`, `card_number`, `atm_name`) VALUES (3, 100, '2022-11-26', '4578015464893204', 'S_064');
INSERT INTO `deposits` (`id`, `amount`, `date`, `card_number`, `atm_name`) VALUES (4, 250, '2022-11-26', '7598150468901754', 'S_064');
INSERT INTO `deposits` (`id`, `amount`, `date`, `card_number`, `atm_name`) VALUES (5, 200, '2022-11-26', '1426045781603457', 'S_064');
INSERT INTO `deposits` (`id`, `amount`, `date`, `card_number`, `atm_name`) VALUES (6, 1000, '2022-11-26', '1426045760345700', 'S_064');
INSERT INTO `deposits` (`id`, `amount`, `date`, `card_number`, `atm_name`) VALUES (7, 450, '2022-11-26', '1426045760345700', 'S_064');
COMMIT;

-- ----------------------------
-- Table structure for transfers
-- ----------------------------
DROP TABLE IF EXISTS `transfers`;
CREATE TABLE `transfers` (
     `id` int(10) unsigned NOT NULL AUTO_INCREMENT,
     `date` date NOT NULL,
     `amount` double unsigned NOT NULL,
     `sent_money` varchar(16) NOT NULL,
     `received_money` varchar(16) NOT NULL,
     PRIMARY KEY (`id`),
     KEY `rc_money_card` (`received_money`),
     KEY `st_money_card` (`sent_money`),
     CONSTRAINT `rc_money_card` FOREIGN KEY (`received_money`) REFERENCES `cards` (`number`),
     CONSTRAINT `st_money_card` FOREIGN KEY (`sent_money`) REFERENCES `cards` (`number`)
) ENGINE=InnoDB AUTO_INCREMENT=6 DEFAULT CHARSET=utf8mb4;

-- ----------------------------
-- Records of transfers
-- ----------------------------
BEGIN;
INSERT INTO `transfers` (`id`, `date`, `amount`, `sent_money`, `received_money`) VALUES (1, '2022-11-26', 1540, '4578015464893204', '1426045781603457');
INSERT INTO `transfers` (`id`, `date`, `amount`, `sent_money`, `received_money`) VALUES (2, '2022-11-26', 450, '7598150468901754', '1426045760345700');
INSERT INTO `transfers` (`id`, `date`, `amount`, `sent_money`, `received_money`) VALUES (3, '2022-11-26', 2600, '8105460479831056', '4578015464893204');
INSERT INTO `transfers` (`id`, `date`, `amount`, `sent_money`, `received_money`) VALUES (4, '2022-11-26', 5000, '1426045760345700', '7598150468901754');
INSERT INTO `transfers` (`id`, `date`, `amount`, `sent_money`, `received_money`) VALUES (5, '2022-11-26', 500, '1426045760345700', '1426045781603457');
COMMIT;

-- ----------------------------
-- Table structure for withdrawals
-- ----------------------------
DROP TABLE IF EXISTS `withdrawals`;
CREATE TABLE `withdrawals` (
    `id` int(10) unsigned NOT NULL AUTO_INCREMENT,
    `amount` double NOT NULL,
    `date` date NOT NULL,
    `atm_name` varchar(255) NOT NULL,
    `card_number` varchar(16) NOT NULL,
    PRIMARY KEY (`id`),
    KEY `atm_name_fk` (`atm_name`),
    KEY `card_number_fk3` (`card_number`),
    CONSTRAINT `atm_name_fk` FOREIGN KEY (`atm_name`) REFERENCES `atms` (`name`),
    CONSTRAINT `card_number_fk3` FOREIGN KEY (`card_number`) REFERENCES `cards` (`number`)
) ENGINE=InnoDB AUTO_INCREMENT=6 DEFAULT CHARSET=utf8mb4;

-- ----------------------------
-- Triggers structure for table cards
-- ----------------------------
DROP TRIGGER IF EXISTS `check_valid`;
delimiter ;;
CREATE TRIGGER `check_valid` BEFORE UPDATE ON `cards` FOR EACH ROW IF OLD.expiration_date <= now() THEN
    SET NEW.expired = 1;
END IF
;;
delimiter ;

-- ----------------------------
-- Triggers structure for table cards
-- ----------------------------
DROP TRIGGER IF EXISTS `check_try`;
delimiter ;;
CREATE TRIGGER `check_try` BEFORE UPDATE ON `cards` FOR EACH ROW IF OLD.try = 3 THEN
    SET NEW.expired = 1;
END IF
;;
delimiter ;

-- ----------------------------
-- Triggers structure for table deposits
-- ----------------------------
DROP TRIGGER IF EXISTS `set_date`;
delimiter ;;
CREATE TRIGGER `set_date` BEFORE INSERT ON `deposits` FOR EACH ROW SET NEW.date = CURTIME()
;;
delimiter ;

-- ----------------------------
-- Triggers structure for table transfers
-- ----------------------------
DROP TRIGGER IF EXISTS `set_date_transfer`;
delimiter ;;
CREATE TRIGGER `set_date_transfer` BEFORE INSERT ON `transfers` FOR EACH ROW SET NEW.date = CURTIME()
;;
delimiter ;

-- ----------------------------
-- Triggers structure for table withdrawals
-- ----------------------------
DROP TRIGGER IF EXISTS `set_date_with`;
delimiter ;;
CREATE TRIGGER `set_date_with` BEFORE INSERT ON `withdrawals` FOR EACH ROW SET NEW.date = CURTIME()
;;
delimiter ;

SET FOREIGN_KEY_CHECKS = 1;
