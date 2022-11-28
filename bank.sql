DROP database if exists `banco`;
Create database `banco`;
use `banco`;
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
INSERT INTO `cards` (`number`, `bank_id`, `cvv`, `nip`, `expiration_date`, `balance`, `type`, `expired`, `try`) VALUES ('1234567890123457', 2, 234, 1234, '2021-11-27', 2323, 'Credit', 0, 0);
INSERT INTO `cards` (`number`, `bank_id`, `cvv`, `nip`, `expiration_date`, `balance`, `type`, `expired`, `try`) VALUES ('1426045760345700', 2, 423, 1233, '2025-11-11', 45215, 'Debit', 0, 0);
INSERT INTO `cards` (`number`, `bank_id`, `cvv`, `nip`, `expiration_date`, `balance`, `type`, `expired`, `try`) VALUES ('1426045781603457', 2, 423, 1233, '2025-11-11', 45215, 'Debit', 0, 0);
INSERT INTO `cards` (`number`, `bank_id`, `cvv`, `nip`, `expiration_date`, `balance`, `type`, `expired`, `try`) VALUES ('340157907902107', 4, 437, 9503, '2024-04-03', 84668.88, 'Credit', 0, 0);
INSERT INTO `cards` (`number`, `bank_id`, `cvv`, `nip`, `expiration_date`, `balance`, `type`, `expired`, `try`) VALUES ('340779047754765', 3, 451, 8695, '2024-08-09', 71145.56, 'Credit', 0, 0);
INSERT INTO `cards` (`number`, `bank_id`, `cvv`, `nip`, `expiration_date`, `balance`, `type`, `expired`, `try`) VALUES ('342640522933076', 2, 216, 5285, '2024-03-09', 55602.67, 'Debit', 0, 0);
INSERT INTO `cards` (`number`, `bank_id`, `cvv`, `nip`, `expiration_date`, `balance`, `type`, `expired`, `try`) VALUES ('343365220660768', 2, 566, 2452, '2024-10-24', 64127.99, 'Debit', 0, 0);
INSERT INTO `cards` (`number`, `bank_id`, `cvv`, `nip`, `expiration_date`, `balance`, `type`, `expired`, `try`) VALUES ('344049301031797', 3, 313, 2842, '2024-01-05', 24770.84, 'Credit', 0, 0);
INSERT INTO `cards` (`number`, `bank_id`, `cvv`, `nip`, `expiration_date`, `balance`, `type`, `expired`, `try`) VALUES ('346850386777776', 2, 471, 4065, '2024-01-25', 18743.08, 'Credit', 0, 0);
INSERT INTO `cards` (`number`, `bank_id`, `cvv`, `nip`, `expiration_date`, `balance`, `type`, `expired`, `try`) VALUES ('347160514691744', 3, 323, 2925, '2023-12-17', 36727.33, 'Debit', 0, 0);
INSERT INTO `cards` (`number`, `bank_id`, `cvv`, `nip`, `expiration_date`, `balance`, `type`, `expired`, `try`) VALUES ('347549945308755', 2, 368, 5274, '2024-01-15', 86118.18, 'Credit', 0, 0);
INSERT INTO `cards` (`number`, `bank_id`, `cvv`, `nip`, `expiration_date`, `balance`, `type`, `expired`, `try`) VALUES ('3533821797813275', 3, 171, 8366, '2024-05-31', 75755.68, 'Credit', 0, 0);
INSERT INTO `cards` (`number`, `bank_id`, `cvv`, `nip`, `expiration_date`, `balance`, `type`, `expired`, `try`) VALUES ('3534379125249992', 3, 226, 5004, '2024-03-20', 29815.15, 'Debit', 0, 0);
INSERT INTO `cards` (`number`, `bank_id`, `cvv`, `nip`, `expiration_date`, `balance`, `type`, `expired`, `try`) VALUES ('3536339427515027', 2, 788, 9956, '2024-02-12', 28563.67, 'Debit', 0, 0);
INSERT INTO `cards` (`number`, `bank_id`, `cvv`, `nip`, `expiration_date`, `balance`, `type`, `expired`, `try`) VALUES ('3536855174531776', 2, 901, 8370, '2024-06-03', 53363.96, 'Credit', 0, 0);
INSERT INTO `cards` (`number`, `bank_id`, `cvv`, `nip`, `expiration_date`, `balance`, `type`, `expired`, `try`) VALUES ('3540277358414758', 2, 326, 3515, '2024-08-06', 85157.63, 'Credit', 0, 0);
INSERT INTO `cards` (`number`, `bank_id`, `cvv`, `nip`, `expiration_date`, `balance`, `type`, `expired`, `try`) VALUES ('3541132133194275', 2, 668, 1642, '2024-01-30', 73399.03, 'Debit', 0, 0);
INSERT INTO `cards` (`number`, `bank_id`, `cvv`, `nip`, `expiration_date`, `balance`, `type`, `expired`, `try`) VALUES ('3543326457004296', 3, 801, 7533, '2024-01-10', 90102.08, 'Debit', 0, 0);
INSERT INTO `cards` (`number`, `bank_id`, `cvv`, `nip`, `expiration_date`, `balance`, `type`, `expired`, `try`) VALUES ('3544195834712254', 4, 261, 3645, '2024-06-15', 11461.92, 'Credit', 0, 0);
INSERT INTO `cards` (`number`, `bank_id`, `cvv`, `nip`, `expiration_date`, `balance`, `type`, `expired`, `try`) VALUES ('3544471253824239', 2, 455, 5662, '2024-05-17', 96740.43, 'Credit', 0, 0);
INSERT INTO `cards` (`number`, `bank_id`, `cvv`, `nip`, `expiration_date`, `balance`, `type`, `expired`, `try`) VALUES ('3546804876812520', 4, 399, 1247, '2023-07-22', 87647.23, 'Debit', 0, 0);
INSERT INTO `cards` (`number`, `bank_id`, `cvv`, `nip`, `expiration_date`, `balance`, `type`, `expired`, `try`) VALUES ('3551743749257426', 3, 307, 3868, '2024-11-02', 31641.34, 'Credit', 0, 0);
INSERT INTO `cards` (`number`, `bank_id`, `cvv`, `nip`, `expiration_date`, `balance`, `type`, `expired`, `try`) VALUES ('3552937551831794', 2, 174, 3052, '2023-11-02', 75332.91, 'Credit', 0, 0);
INSERT INTO `cards` (`number`, `bank_id`, `cvv`, `nip`, `expiration_date`, `balance`, `type`, `expired`, `try`) VALUES ('3557595847148012', 4, 432, 9393, '2024-09-25', 18025.08, 'Credit', 0, 0);
INSERT INTO `cards` (`number`, `bank_id`, `cvv`, `nip`, `expiration_date`, `balance`, `type`, `expired`, `try`) VALUES ('3560797919681673', 3, 843, 5283, '2023-03-24', 82040.22, 'Debit', 0, 0);
INSERT INTO `cards` (`number`, `bank_id`, `cvv`, `nip`, `expiration_date`, `balance`, `type`, `expired`, `try`) VALUES ('3562353412328277', 2, 738, 3638, '2023-03-31', 56245.27, 'Credit', 0, 0);
INSERT INTO `cards` (`number`, `bank_id`, `cvv`, `nip`, `expiration_date`, `balance`, `type`, `expired`, `try`) VALUES ('3563446572483616', 2, 421, 2337, '2023-03-31', 24436.12, 'Credit', 0, 0);
INSERT INTO `cards` (`number`, `bank_id`, `cvv`, `nip`, `expiration_date`, `balance`, `type`, `expired`, `try`) VALUES ('3563807515400357', 2, 483, 5773, '2024-01-20', 39755.6, 'Debit', 0, 0);
INSERT INTO `cards` (`number`, `bank_id`, `cvv`, `nip`, `expiration_date`, `balance`, `type`, `expired`, `try`) VALUES ('3564889708842545', 3, 654, 4159, '2023-08-28', 97510.77, 'Credit', 0, 0);
INSERT INTO `cards` (`number`, `bank_id`, `cvv`, `nip`, `expiration_date`, `balance`, `type`, `expired`, `try`) VALUES ('3569191270336054', 4, 994, 5015, '2023-05-14', 67144.91, 'Credit', 0, 0);
INSERT INTO `cards` (`number`, `bank_id`, `cvv`, `nip`, `expiration_date`, `balance`, `type`, `expired`, `try`) VALUES ('3571008299256327', 4, 171, 5228, '2023-07-11', 74330.04, 'Credit', 0, 0);
INSERT INTO `cards` (`number`, `bank_id`, `cvv`, `nip`, `expiration_date`, `balance`, `type`, `expired`, `try`) VALUES ('3575169204282075', 2, 875, 4301, '2023-02-05', 72305.03, 'Debit', 0, 0);
INSERT INTO `cards` (`number`, `bank_id`, `cvv`, `nip`, `expiration_date`, `balance`, `type`, `expired`, `try`) VALUES ('3579729972433170', 4, 665, 5600, '2024-03-28', 79681.86, 'Debit', 0, 0);
INSERT INTO `cards` (`number`, `bank_id`, `cvv`, `nip`, `expiration_date`, `balance`, `type`, `expired`, `try`) VALUES ('3582999250446443', 2, 753, 5085, '2023-03-10', 32932.14, 'Credit', 0, 0);
INSERT INTO `cards` (`number`, `bank_id`, `cvv`, `nip`, `expiration_date`, `balance`, `type`, `expired`, `try`) VALUES ('370246494933105', 4, 583, 7150, '2023-04-10', 42515.42, 'Debit', 0, 0);
INSERT INTO `cards` (`number`, `bank_id`, `cvv`, `nip`, `expiration_date`, `balance`, `type`, `expired`, `try`) VALUES ('370299607958063', 4, 310, 5275, '2024-06-28', 17037.29, 'Credit', 0, 0);
INSERT INTO `cards` (`number`, `bank_id`, `cvv`, `nip`, `expiration_date`, `balance`, `type`, `expired`, `try`) VALUES ('371074310936311', 2, 744, 5809, '2024-09-24', 14781.2, 'Debit', 0, 0);
INSERT INTO `cards` (`number`, `bank_id`, `cvv`, `nip`, `expiration_date`, `balance`, `type`, `expired`, `try`) VALUES ('372628600140626', 3, 581, 6783, '2023-04-02', 20088.41, 'Credit', 0, 0);
INSERT INTO `cards` (`number`, `bank_id`, `cvv`, `nip`, `expiration_date`, `balance`, `type`, `expired`, `try`) VALUES ('374750571238052', 2, 729, 1398, '2023-01-14', 97392.58, 'Debit', 0, 0);
INSERT INTO `cards` (`number`, `bank_id`, `cvv`, `nip`, `expiration_date`, `balance`, `type`, `expired`, `try`) VALUES ('375739030335772', 3, 881, 2914, '2023-07-16', 16663.34, 'Credit', 0, 0);
INSERT INTO `cards` (`number`, `bank_id`, `cvv`, `nip`, `expiration_date`, `balance`, `type`, `expired`, `try`) VALUES ('377844129617821', 3, 286, 8097, '2023-10-27', 86668.4, 'Credit', 0, 0);
INSERT INTO `cards` (`number`, `bank_id`, `cvv`, `nip`, `expiration_date`, `balance`, `type`, `expired`, `try`) VALUES ('377941154739635', 2, 525, 5041, '2023-12-19', 96047.09, 'Credit', 0, 0);
INSERT INTO `cards` (`number`, `bank_id`, `cvv`, `nip`, `expiration_date`, `balance`, `type`, `expired`, `try`) VALUES ('379187255455808', 3, 622, 2926, '2023-04-03', 72378.39, 'Credit', 0, 0);
INSERT INTO `cards` (`number`, `bank_id`, `cvv`, `nip`, `expiration_date`, `balance`, `type`, `expired`, `try`) VALUES ('4000852670853387', 2, 177, 5073, '2024-10-23', 73931.21, 'Debit', 0, 0);
INSERT INTO `cards` (`number`, `bank_id`, `cvv`, `nip`, `expiration_date`, `balance`, `type`, `expired`, `try`) VALUES ('4245382455732627', 2, 665, 4139, '2023-11-08', 89816.78, 'Debit', 0, 0);
INSERT INTO `cards` (`number`, `bank_id`, `cvv`, `nip`, `expiration_date`, `balance`, `type`, `expired`, `try`) VALUES ('4264651589461055', 4, 226, 9375, '2023-08-05', 95761.63, 'Debit', 0, 0);
INSERT INTO `cards` (`number`, `bank_id`, `cvv`, `nip`, `expiration_date`, `balance`, `type`, `expired`, `try`) VALUES ('4286229168107425', 4, 410, 2754, '2023-04-17', 28882.47, 'Credit', 0, 0);
INSERT INTO `cards` (`number`, `bank_id`, `cvv`, `nip`, `expiration_date`, `balance`, `type`, `expired`, `try`) VALUES ('4293032601455240', 3, 785, 1558, '2024-03-14', 47256.9, 'Credit', 0, 0);
INSERT INTO `cards` (`number`, `bank_id`, `cvv`, `nip`, `expiration_date`, `balance`, `type`, `expired`, `try`) VALUES ('4314926655625134', 2, 327, 5565, '2023-02-04', 90986.04, 'Debit', 0, 0);
INSERT INTO `cards` (`number`, `bank_id`, `cvv`, `nip`, `expiration_date`, `balance`, `type`, `expired`, `try`) VALUES ('4371175609799641', 4, 254, 5437, '2024-06-29', 54439.54, 'Credit', 0, 0);
INSERT INTO `cards` (`number`, `bank_id`, `cvv`, `nip`, `expiration_date`, `balance`, `type`, `expired`, `try`) VALUES ('4380368973378899', 2, 504, 7567, '2024-05-04', 21998.38, 'Credit', 0, 0);
INSERT INTO `cards` (`number`, `bank_id`, `cvv`, `nip`, `expiration_date`, `balance`, `type`, `expired`, `try`) VALUES ('4465724455073572', 2, 819, 3332, '2024-04-13', 83764.67, 'Credit', 0, 0);
INSERT INTO `cards` (`number`, `bank_id`, `cvv`, `nip`, `expiration_date`, `balance`, `type`, `expired`, `try`) VALUES ('4480648722767107', 3, 617, 1095, '2024-07-04', 44773.94, 'Credit', 0, 0);
INSERT INTO `cards` (`number`, `bank_id`, `cvv`, `nip`, `expiration_date`, `balance`, `type`, `expired`, `try`) VALUES ('4489036384069874', 3, 467, 2217, '2024-04-21', 45390.55, 'Debit', 0, 0);
INSERT INTO `cards` (`number`, `bank_id`, `cvv`, `nip`, `expiration_date`, `balance`, `type`, `expired`, `try`) VALUES ('4526849118434197', 2, 743, 1463, '2023-07-24', 10251.27, 'Credit', 0, 0);
INSERT INTO `cards` (`number`, `bank_id`, `cvv`, `nip`, `expiration_date`, `balance`, `type`, `expired`, `try`) VALUES ('4578015464893204', 2, 130, 1233, '2025-06-23', 17590, 'Credit', 0, 0);
INSERT INTO `cards` (`number`, `bank_id`, `cvv`, `nip`, `expiration_date`, `balance`, `type`, `expired`, `try`) VALUES ('4587880260037039', 4, 122, 9084, '2024-05-06', 92174.6, 'Debit', 0, 0);
INSERT INTO `cards` (`number`, `bank_id`, `cvv`, `nip`, `expiration_date`, `balance`, `type`, `expired`, `try`) VALUES ('4678073088081583', 2, 784, 4329, '2024-10-17', 12676.84, 'Debit', 0, 0);
INSERT INTO `cards` (`number`, `bank_id`, `cvv`, `nip`, `expiration_date`, `balance`, `type`, `expired`, `try`) VALUES ('4744741037451038', 2, 373, 1505, '2023-06-28', 26317.6, 'Credit', 0, 0);
INSERT INTO `cards` (`number`, `bank_id`, `cvv`, `nip`, `expiration_date`, `balance`, `type`, `expired`, `try`) VALUES ('4784173674787071', 2, 846, 6904, '2023-07-30', 95936.82, 'Debit', 0, 0);
INSERT INTO `cards` (`number`, `bank_id`, `cvv`, `nip`, `expiration_date`, `balance`, `type`, `expired`, `try`) VALUES ('4860874760539048', 2, 544, 8216, '2023-01-08', 34828.99, 'Debit', 0, 0);
INSERT INTO `cards` (`number`, `bank_id`, `cvv`, `nip`, `expiration_date`, `balance`, `type`, `expired`, `try`) VALUES ('4955252418340133', 3, 425, 3072, '2024-04-03', 67377.25, 'Credit', 0, 0);
INSERT INTO `cards` (`number`, `bank_id`, `cvv`, `nip`, `expiration_date`, `balance`, `type`, `expired`, `try`) VALUES ('5006234843759703', 4, 502, 1530, '2023-06-18', 59410.91, 'Debit', 0, 0);
INSERT INTO `cards` (`number`, `bank_id`, `cvv`, `nip`, `expiration_date`, `balance`, `type`, `expired`, `try`) VALUES ('5037179559442619', 3, 171, 5907, '2023-05-04', 95900.17, 'Credit', 0, 0);
INSERT INTO `cards` (`number`, `bank_id`, `cvv`, `nip`, `expiration_date`, `balance`, `type`, `expired`, `try`) VALUES ('5043472012723721', 4, 460, 3099, '2023-05-04', 42054.03, 'Debit', 0, 0);
INSERT INTO `cards` (`number`, `bank_id`, `cvv`, `nip`, `expiration_date`, `balance`, `type`, `expired`, `try`) VALUES ('5081512612248174', 3, 882, 2392, '2023-09-22', 12359.79, 'Debit', 0, 0);
INSERT INTO `cards` (`number`, `bank_id`, `cvv`, `nip`, `expiration_date`, `balance`, `type`, `expired`, `try`) VALUES ('5089770985719872', 4, 295, 7553, '2024-03-03', 52106.56, 'Credit', 0, 0);
INSERT INTO `cards` (`number`, `bank_id`, `cvv`, `nip`, `expiration_date`, `balance`, `type`, `expired`, `try`) VALUES ('5093798411380799', 2, 887, 3513, '2024-04-30', 61201.99, 'Debit', 0, 0);
INSERT INTO `cards` (`number`, `bank_id`, `cvv`, `nip`, `expiration_date`, `balance`, `type`, `expired`, `try`) VALUES ('5113626797107856', 4, 435, 6009, '2023-09-26', 50942.44, 'Credit', 0, 0);
INSERT INTO `cards` (`number`, `bank_id`, `cvv`, `nip`, `expiration_date`, `balance`, `type`, `expired`, `try`) VALUES ('5148217205964390', 4, 807, 9532, '2023-06-05', 75787.66, 'Debit', 0, 0);
INSERT INTO `cards` (`number`, `bank_id`, `cvv`, `nip`, `expiration_date`, `balance`, `type`, `expired`, `try`) VALUES ('5177227136378879', 2, 387, 9044, '2024-06-11', 62013.19, 'Credit', 0, 0);
INSERT INTO `cards` (`number`, `bank_id`, `cvv`, `nip`, `expiration_date`, `balance`, `type`, `expired`, `try`) VALUES ('5185539167473944', 2, 851, 9086, '2023-10-07', 41123.31, 'Debit', 0, 0);
INSERT INTO `cards` (`number`, `bank_id`, `cvv`, `nip`, `expiration_date`, `balance`, `type`, `expired`, `try`) VALUES ('5270495933966040', 2, 625, 1742, '2023-10-11', 44593.09, 'Debit', 0, 0);
INSERT INTO `cards` (`number`, `bank_id`, `cvv`, `nip`, `expiration_date`, `balance`, `type`, `expired`, `try`) VALUES ('5317024703284189', 3, 965, 4044, '2023-04-10', 71846.08, 'Debit', 0, 0);
INSERT INTO `cards` (`number`, `bank_id`, `cvv`, `nip`, `expiration_date`, `balance`, `type`, `expired`, `try`) VALUES ('5329884835842782', 4, 766, 7131, '2024-01-12', 36623.17, 'Credit', 0, 0);
INSERT INTO `cards` (`number`, `bank_id`, `cvv`, `nip`, `expiration_date`, `balance`, `type`, `expired`, `try`) VALUES ('5393328546917559', 3, 972, 3454, '2023-11-24', 58065.5, 'Debit', 0, 0);
INSERT INTO `cards` (`number`, `bank_id`, `cvv`, `nip`, `expiration_date`, `balance`, `type`, `expired`, `try`) VALUES ('5432358970110994', 3, 739, 9635, '2024-07-25', 74366.87, 'Debit', 0, 0);
INSERT INTO `cards` (`number`, `bank_id`, `cvv`, `nip`, `expiration_date`, `balance`, `type`, `expired`, `try`) VALUES ('5451722917573604', 2, 277, 4140, '2023-06-29', 40017.98, 'Debit', 0, 0);
INSERT INTO `cards` (`number`, `bank_id`, `cvv`, `nip`, `expiration_date`, `balance`, `type`, `expired`, `try`) VALUES ('5460291941439397', 2, 399, 8057, '2023-05-25', 11585.42, 'Credit', 0, 0);
INSERT INTO `cards` (`number`, `bank_id`, `cvv`, `nip`, `expiration_date`, `balance`, `type`, `expired`, `try`) VALUES ('5532081857517169', 3, 815, 9599, '2023-06-09', 20307.97, 'Debit', 0, 0);
INSERT INTO `cards` (`number`, `bank_id`, `cvv`, `nip`, `expiration_date`, `balance`, `type`, `expired`, `try`) VALUES ('5534717431312845', 2, 884, 3437, '2023-03-27', 36299.84, 'Credit', 0, 0);
INSERT INTO `cards` (`number`, `bank_id`, `cvv`, `nip`, `expiration_date`, `balance`, `type`, `expired`, `try`) VALUES ('5539995213950868', 4, 546, 8678, '2023-05-01', 53330.96, 'Credit', 0, 0);
INSERT INTO `cards` (`number`, `bank_id`, `cvv`, `nip`, `expiration_date`, `balance`, `type`, `expired`, `try`) VALUES ('5552341989067789', 3, 365, 9171, '2023-02-28', 34262.99, 'Credit', 0, 0);
INSERT INTO `cards` (`number`, `bank_id`, `cvv`, `nip`, `expiration_date`, `balance`, `type`, `expired`, `try`) VALUES ('5556385560239669', 3, 575, 7342, '2024-11-21', 32063.74, 'Credit', 0, 0);
INSERT INTO `cards` (`number`, `bank_id`, `cvv`, `nip`, `expiration_date`, `balance`, `type`, `expired`, `try`) VALUES ('6201020248284194', 4, 632, 6575, '2023-03-22', 39895.45, 'Credit', 0, 0);
INSERT INTO `cards` (`number`, `bank_id`, `cvv`, `nip`, `expiration_date`, `balance`, `type`, `expired`, `try`) VALUES ('6201583914509178', 4, 282, 1382, '2023-02-13', 48675.64, 'Debit', 0, 0);
INSERT INTO `cards` (`number`, `bank_id`, `cvv`, `nip`, `expiration_date`, `balance`, `type`, `expired`, `try`) VALUES ('6205523334355543', 2, 900, 9116, '2024-04-03', 89545.13, 'Credit', 0, 0);
INSERT INTO `cards` (`number`, `bank_id`, `cvv`, `nip`, `expiration_date`, `balance`, `type`, `expired`, `try`) VALUES ('6213859805893731', 3, 488, 7456, '2023-03-26', 42545.18, 'Debit', 0, 0);
INSERT INTO `cards` (`number`, `bank_id`, `cvv`, `nip`, `expiration_date`, `balance`, `type`, `expired`, `try`) VALUES ('6219192588092907', 4, 544, 7378, '2023-03-06', 12651.61, 'Debit', 0, 0);
INSERT INTO `cards` (`number`, `bank_id`, `cvv`, `nip`, `expiration_date`, `balance`, `type`, `expired`, `try`) VALUES ('6220674525815489', 4, 805, 3431, '2024-05-11', 85474.33, 'Credit', 0, 0);
INSERT INTO `cards` (`number`, `bank_id`, `cvv`, `nip`, `expiration_date`, `balance`, `type`, `expired`, `try`) VALUES ('6221989210477281', 3, 958, 8542, '2024-08-09', 81355.74, 'Credit', 0, 0);
INSERT INTO `cards` (`number`, `bank_id`, `cvv`, `nip`, `expiration_date`, `balance`, `type`, `expired`, `try`) VALUES ('6222486987359502', 2, 947, 3002, '2023-12-02', 26687.6, 'Credit', 0, 0);
INSERT INTO `cards` (`number`, `bank_id`, `cvv`, `nip`, `expiration_date`, `balance`, `type`, `expired`, `try`) VALUES ('6225067411483261', 2, 710, 2586, '2024-07-26', 11612.48, 'Credit', 0, 0);
INSERT INTO `cards` (`number`, `bank_id`, `cvv`, `nip`, `expiration_date`, `balance`, `type`, `expired`, `try`) VALUES ('6229161765733243', 2, 921, 4476, '2024-04-08', 21928.29, 'Credit', 0, 0);
INSERT INTO `cards` (`number`, `bank_id`, `cvv`, `nip`, `expiration_date`, `balance`, `type`, `expired`, `try`) VALUES ('6230727846792197', 3, 572, 4604, '2024-05-05', 73371.23, 'Debit', 0, 0);
INSERT INTO `cards` (`number`, `bank_id`, `cvv`, `nip`, `expiration_date`, `balance`, `type`, `expired`, `try`) VALUES ('6232021513489057', 4, 761, 7349, '2024-03-16', 57349.47, 'Debit', 0, 0);
INSERT INTO `cards` (`number`, `bank_id`, `cvv`, `nip`, `expiration_date`, `balance`, `type`, `expired`, `try`) VALUES ('6239306717945878', 3, 688, 5794, '2024-10-12', 70101.83, 'Credit', 0, 0);
INSERT INTO `cards` (`number`, `bank_id`, `cvv`, `nip`, `expiration_date`, `balance`, `type`, `expired`, `try`) VALUES ('6249926993750706', 4, 244, 9657, '2023-06-01', 86192.24, 'Credit', 0, 0);
INSERT INTO `cards` (`number`, `bank_id`, `cvv`, `nip`, `expiration_date`, `balance`, `type`, `expired`, `try`) VALUES ('6254621063659158', 2, 472, 1428, '2023-11-11', 60941.97, 'Credit', 0, 0);
INSERT INTO `cards` (`number`, `bank_id`, `cvv`, `nip`, `expiration_date`, `balance`, `type`, `expired`, `try`) VALUES ('6258281669975213', 4, 512, 7859, '2024-07-27', 13198.91, 'Credit', 0, 0);
INSERT INTO `cards` (`number`, `bank_id`, `cvv`, `nip`, `expiration_date`, `balance`, `type`, `expired`, `try`) VALUES ('6258620234092196', 4, 278, 9317, '2024-01-26', 70452.95, 'Credit', 0, 0);
INSERT INTO `cards` (`number`, `bank_id`, `cvv`, `nip`, `expiration_date`, `balance`, `type`, `expired`, `try`) VALUES ('6267908893742766', 4, 535, 7454, '2024-01-16', 95397.56, 'Debit', 0, 0);
INSERT INTO `cards` (`number`, `bank_id`, `cvv`, `nip`, `expiration_date`, `balance`, `type`, `expired`, `try`) VALUES ('6276494454809533', 2, 524, 8211, '2023-06-09', 91886.83, 'Debit', 0, 0);
INSERT INTO `cards` (`number`, `bank_id`, `cvv`, `nip`, `expiration_date`, `balance`, `type`, `expired`, `try`) VALUES ('6287120193221873', 2, 486, 5736, '2024-03-11', 13149.41, 'Credit', 0, 0);
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
INSERT INTO `customers` (`id`, `name`, `surname`, `card_number`) VALUES (6, 'Earl', 'Tucker', '3563807515400357');
INSERT INTO `customers` (`id`, `name`, `surname`, `card_number`) VALUES (7, 'Juan', 'Stewart', '6258281669975213');
INSERT INTO `customers` (`id`, `name`, `surname`, `card_number`) VALUES (8, 'Eleanor', 'Bell', '4678073088081583');
INSERT INTO `customers` (`id`, `name`, `surname`, `card_number`) VALUES (9, 'Jacob', 'Johnson', '5432358970110994');
INSERT INTO `customers` (`id`, `name`, `surname`, `card_number`) VALUES (10, 'Norman', 'Alexander', '5539995213950868');
INSERT INTO `customers` (`id`, `name`, `surname`, `card_number`) VALUES (11, 'Alexander', 'Sanchez', '6201583914509178');
INSERT INTO `customers` (`id`, `name`, `surname`, `card_number`) VALUES (12, 'Joan', 'Kim', '3582999250446443');
INSERT INTO `customers` (`id`, `name`, `surname`, `card_number`) VALUES (13, 'Andrea', 'Hill', '6287120193221873');
INSERT INTO `customers` (`id`, `name`, `surname`, `card_number`) VALUES (14, 'Luis', 'Guzman', '370299607958063');
INSERT INTO `customers` (`id`, `name`, `surname`, `card_number`) VALUES (15, 'Susan', 'Garcia', '6254621063659158');
INSERT INTO `customers` (`id`, `name`, `surname`, `card_number`) VALUES (16, 'Victor', 'Sanders', '6267908893742766');
INSERT INTO `customers` (`id`, `name`, `surname`, `card_number`) VALUES (17, 'Francis', 'Ferguson', '6258620234092196');
INSERT INTO `customers` (`id`, `name`, `surname`, `card_number`) VALUES (18, 'John', 'Freeman', '3533821797813275');
INSERT INTO `customers` (`id`, `name`, `surname`, `card_number`) VALUES (19, 'Allen', 'Campbell', '5177227136378879');
INSERT INTO `customers` (`id`, `name`, `surname`, `card_number`) VALUES (20, 'Shannon', 'Kelley', '3575169204282075');
INSERT INTO `customers` (`id`, `name`, `surname`, `card_number`) VALUES (21, 'Katherine', 'Soto', '5093798411380799');
INSERT INTO `customers` (`id`, `name`, `surname`, `card_number`) VALUES (22, 'Jerry', 'Clark', '3560797919681673');
INSERT INTO `customers` (`id`, `name`, `surname`, `card_number`) VALUES (23, 'Steve', 'Ramirez', '5043472012723721');
INSERT INTO `customers` (`id`, `name`, `surname`, `card_number`) VALUES (24, 'Ricky', 'Ward', '3543326457004296');
INSERT INTO `customers` (`id`, `name`, `surname`, `card_number`) VALUES (25, 'Curtis', 'Henry', '340779047754765');
INSERT INTO `customers` (`id`, `name`, `surname`, `card_number`) VALUES (26, 'Walter', 'Romero', '372628600140626');
INSERT INTO `customers` (`id`, `name`, `surname`, `card_number`) VALUES (27, 'Jacob', 'Olson', '371074310936311');
INSERT INTO `customers` (`id`, `name`, `surname`, `card_number`) VALUES (28, 'Hazel', 'Salazar', '3534379125249992');
INSERT INTO `customers` (`id`, `name`, `surname`, `card_number`) VALUES (29, 'Jacqueline', 'Rice', '3571008299256327');
INSERT INTO `customers` (`id`, `name`, `surname`, `card_number`) VALUES (30, 'Sara', 'Payne', '5081512612248174');
INSERT INTO `customers` (`id`, `name`, `surname`, `card_number`) VALUES (31, 'Bernard', 'James', '5185539167473944');
INSERT INTO `customers` (`id`, `name`, `surname`, `card_number`) VALUES (32, 'Richard', 'Harrison', '3579729972433170');
INSERT INTO `customers` (`id`, `name`, `surname`, `card_number`) VALUES (33, 'Micheal', 'Black', '6230727846792197');
INSERT INTO `customers` (`id`, `name`, `surname`, `card_number`) VALUES (34, 'Doris', 'Spencer', '342640522933076');
INSERT INTO `customers` (`id`, `name`, `surname`, `card_number`) VALUES (35, 'Karen', 'Bennett', '5460291941439397');
INSERT INTO `customers` (`id`, `name`, `surname`, `card_number`) VALUES (36, 'Jeffrey', 'Powell', '3544195834712254');
INSERT INTO `customers` (`id`, `name`, `surname`, `card_number`) VALUES (37, 'Carol', 'Campbell', '5534717431312845');
INSERT INTO `customers` (`id`, `name`, `surname`, `card_number`) VALUES (38, 'Heather', 'King', '6205523334355543');
INSERT INTO `customers` (`id`, `name`, `surname`, `card_number`) VALUES (39, 'Jacob', 'Ruiz', '1426045781603457');
INSERT INTO `customers` (`id`, `name`, `surname`, `card_number`) VALUES (40, 'Leslie', 'Vargas', '5006234843759703');
INSERT INTO `customers` (`id`, `name`, `surname`, `card_number`) VALUES (41, 'Annie', 'Kelly', '5317024703284189');
INSERT INTO `customers` (`id`, `name`, `surname`, `card_number`) VALUES (42, 'Janet', 'Carter', '6232021513489057');
INSERT INTO `customers` (`id`, `name`, `surname`, `card_number`) VALUES (43, 'George', 'Burns', '4245382455732627');
INSERT INTO `customers` (`id`, `name`, `surname`, `card_number`) VALUES (44, 'Stephanie', 'Price', '6249926993750706');
INSERT INTO `customers` (`id`, `name`, `surname`, `card_number`) VALUES (45, 'Victor', 'Mitchell', '3552937551831794');
INSERT INTO `customers` (`id`, `name`, `surname`, `card_number`) VALUES (46, 'Arthur', 'Fernandez', '8105460479831056');
INSERT INTO `customers` (`id`, `name`, `surname`, `card_number`) VALUES (47, 'Bruce', 'Nichols', '1234567890123457');
INSERT INTO `customers` (`id`, `name`, `surname`, `card_number`) VALUES (48, 'Sandra', 'Reynolds', '370246494933105');
INSERT INTO `customers` (`id`, `name`, `surname`, `card_number`) VALUES (49, 'Sara', 'Nguyen', '5552341989067789');
INSERT INTO `customers` (`id`, `name`, `surname`, `card_number`) VALUES (50, 'Mike', 'Patterson', '4489036384069874');
INSERT INTO `customers` (`id`, `name`, `surname`, `card_number`) VALUES (51, 'Sandra', 'Perry', '6276494454809533');
INSERT INTO `customers` (`id`, `name`, `surname`, `card_number`) VALUES (52, 'Bruce', 'Cox', '5329884835842782');
INSERT INTO `customers` (`id`, `name`, `surname`, `card_number`) VALUES (53, 'Melissa', 'Butler', '3551743749257426');
INSERT INTO `customers` (`id`, `name`, `surname`, `card_number`) VALUES (54, 'Danielle', 'Harrison', '347549945308755');
INSERT INTO `customers` (`id`, `name`, `surname`, `card_number`) VALUES (55, 'David', 'Crawford', '1426045760345700');
INSERT INTO `customers` (`id`, `name`, `surname`, `card_number`) VALUES (56, 'Edna', 'Green', '5270495933966040');
INSERT INTO `customers` (`id`, `name`, `surname`, `card_number`) VALUES (57, 'Sean', 'Rodriguez', '4000852670853387');
INSERT INTO `customers` (`id`, `name`, `surname`, `card_number`) VALUES (58, 'Marcus', 'Wood', '3564889708842545');
INSERT INTO `customers` (`id`, `name`, `surname`, `card_number`) VALUES (59, 'Barbara', 'Stephens', '4784173674787071');
INSERT INTO `customers` (`id`, `name`, `surname`, `card_number`) VALUES (60, 'Howard', 'Anderson', '4526849118434197');
INSERT INTO `customers` (`id`, `name`, `surname`, `card_number`) VALUES (61, 'George', 'Hill', '6225067411483261');
INSERT INTO `customers` (`id`, `name`, `surname`, `card_number`) VALUES (62, 'Harry', 'Gardner', '5556385560239669');
INSERT INTO `customers` (`id`, `name`, `surname`, `card_number`) VALUES (63, 'Benjamin', 'Murphy', '4587880260037039');
INSERT INTO `customers` (`id`, `name`, `surname`, `card_number`) VALUES (64, 'Bobby', 'Warren', '374750571238052');
INSERT INTO `customers` (`id`, `name`, `surname`, `card_number`) VALUES (65, 'Richard', 'Boyd', '5148217205964390');
INSERT INTO `customers` (`id`, `name`, `surname`, `card_number`) VALUES (66, 'Jeffrey', 'Jenkins', '3563446572483616');
INSERT INTO `customers` (`id`, `name`, `surname`, `card_number`) VALUES (67, 'Kyle', 'Anderson', '4955252418340133');
INSERT INTO `customers` (`id`, `name`, `surname`, `card_number`) VALUES (68, 'Timothy', 'Murphy', '3541132133194275');
INSERT INTO `customers` (`id`, `name`, `surname`, `card_number`) VALUES (69, 'Raymond', 'Gonzales', '5532081857517169');
INSERT INTO `customers` (`id`, `name`, `surname`, `card_number`) VALUES (70, 'Jane', 'Washington', '4293032601455240');
INSERT INTO `customers` (`id`, `name`, `surname`, `card_number`) VALUES (71, 'Randy', 'Anderson', '5451722917573604');
INSERT INTO `customers` (`id`, `name`, `surname`, `card_number`) VALUES (72, 'Ruby', 'Roberts', '6213859805893731');
INSERT INTO `customers` (`id`, `name`, `surname`, `card_number`) VALUES (73, 'Rosa', 'Rodriguez', '344049301031797');
INSERT INTO `customers` (`id`, `name`, `surname`, `card_number`) VALUES (74, 'Kathleen', 'Snyder', '4465724455073572');
INSERT INTO `customers` (`id`, `name`, `surname`, `card_number`) VALUES (75, 'Rose', 'Jones', '347160514691744');
INSERT INTO `customers` (`id`, `name`, `surname`, `card_number`) VALUES (76, 'Rosa', 'Kennedy', '5089770985719872');
INSERT INTO `customers` (`id`, `name`, `surname`, `card_number`) VALUES (77, 'Bobby', 'Martin', '375739030335772');
INSERT INTO `customers` (`id`, `name`, `surname`, `card_number`) VALUES (78, 'Troy', 'Grant', '3557595847148012');
INSERT INTO `customers` (`id`, `name`, `surname`, `card_number`) VALUES (79, 'Lawrence', 'Walker', '3536855174531776');
INSERT INTO `customers` (`id`, `name`, `surname`, `card_number`) VALUES (80, 'Debbie', 'Myers', '4380368973378899');
INSERT INTO `customers` (`id`, `name`, `surname`, `card_number`) VALUES (81, 'Ryan', 'Sanders', '3569191270336054');
INSERT INTO `customers` (`id`, `name`, `surname`, `card_number`) VALUES (82, 'Eleanor', 'Reyes', '377844129617821');
INSERT INTO `customers` (`id`, `name`, `surname`, `card_number`) VALUES (83, 'Howard', 'Flores', '3562353412328277');
INSERT INTO `customers` (`id`, `name`, `surname`, `card_number`) VALUES (84, 'Christopher', 'White', '5113626797107856');
INSERT INTO `customers` (`id`, `name`, `surname`, `card_number`) VALUES (85, 'Micheal', 'Porter', '4371175609799641');
INSERT INTO `customers` (`id`, `name`, `surname`, `card_number`) VALUES (86, 'Mike', 'Woods', '3546804876812520');
INSERT INTO `customers` (`id`, `name`, `surname`, `card_number`) VALUES (87, 'Antonio', 'Ford', '4480648722767107');
INSERT INTO `customers` (`id`, `name`, `surname`, `card_number`) VALUES (88, 'Bernard', 'Harris', '6201020248284194');
INSERT INTO `customers` (`id`, `name`, `surname`, `card_number`) VALUES (89, 'Wanda', 'Coleman', '6220674525815489');
INSERT INTO `customers` (`id`, `name`, `surname`, `card_number`) VALUES (90, 'Melvin', 'Salazar', '7598150468901754');
INSERT INTO `customers` (`id`, `name`, `surname`, `card_number`) VALUES (91, 'Francisco', 'Reynolds', '5037179559442619');
INSERT INTO `customers` (`id`, `name`, `surname`, `card_number`) VALUES (92, 'Willie', 'Sanchez', '340157907902107');
INSERT INTO `customers` (`id`, `name`, `surname`, `card_number`) VALUES (93, 'Richard', 'Flores', '6219192588092907');
INSERT INTO `customers` (`id`, `name`, `surname`, `card_number`) VALUES (94, 'Cynthia', 'Bennett', '343365220660768');
INSERT INTO `customers` (`id`, `name`, `surname`, `card_number`) VALUES (95, 'Beverly', 'Porter', '6221989210477281');
INSERT INTO `customers` (`id`, `name`, `surname`, `card_number`) VALUES (96, 'Jessica', 'Meyer', '3544471253824239');
INSERT INTO `customers` (`id`, `name`, `surname`, `card_number`) VALUES (97, 'Robert', 'Carter', '4578015464893204');
INSERT INTO `customers` (`id`, `name`, `surname`, `card_number`) VALUES (98, 'Florence', 'Alexander', '6222486987359502');
INSERT INTO `customers` (`id`, `name`, `surname`, `card_number`) VALUES (99, 'Antonio', 'Washington', '4264651589461055');
INSERT INTO `customers` (`id`, `name`, `surname`, `card_number`) VALUES (100, 'Miguel', 'Henderson', '346850386777776');
INSERT INTO `customers` (`id`, `name`, `surname`, `card_number`) VALUES (101, 'Martha', 'Robinson', '6229161765733243');
INSERT INTO `customers` (`id`, `name`, `surname`, `card_number`) VALUES (102, 'Andrea', 'Nichols', '379187255455808');
INSERT INTO `customers` (`id`, `name`, `surname`, `card_number`) VALUES (103, 'Thelma', 'Thomas', '4744741037451038');
INSERT INTO `customers` (`id`, `name`, `surname`, `card_number`) VALUES (104, 'Bernard', 'Hicks', '3540277358414758');
INSERT INTO `customers` (`id`, `name`, `surname`, `card_number`) VALUES (105, 'Dorothy', 'Schmidt', '4314926655625134');
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
-- Table structure for deudas
-- ----------------------------
DROP TABLE IF EXISTS `deudas`;
CREATE TABLE `deudas` (
    `id` int(10) unsigned NOT NULL AUTO_INCREMENT,
    `number` varchar(16) NOT NULL,
    `type` varchar(10) NOT NULL,
    `deuda` float NOT NULL,
    PRIMARY KEY (`id`),
    KEY `number_deudas_fk` (`number`),
    CONSTRAINT `number_deudas_fk` FOREIGN KEY (`number`) REFERENCES `cards` (`number`)
) ENGINE=InnoDB AUTO_INCREMENT=61 DEFAULT CHARSET=utf8mb4;

-- ----------------------------
-- Records of deudas
-- ----------------------------
BEGIN;
INSERT INTO `deudas` (`id`, `number`, `type`, `deuda`) VALUES (1, '4578015464893204', 'Credit', 0);
INSERT INTO `deudas` (`id`, `number`, `type`, `deuda`) VALUES (2, '7598150468901754', 'Credit', 0);
INSERT INTO `deudas` (`id`, `number`, `type`, `deuda`) VALUES (3, '1234567890123457', 'Credit', 0);
INSERT INTO `deudas` (`id`, `number`, `type`, `deuda`) VALUES (4, '3564889708842545', 'Credit', 0);
INSERT INTO `deudas` (`id`, `number`, `type`, `deuda`) VALUES (5, '3551743749257426', 'Credit', 0);
INSERT INTO `deudas` (`id`, `number`, `type`, `deuda`) VALUES (6, '346850386777776', 'Credit', 0);
INSERT INTO `deudas` (`id`, `number`, `type`, `deuda`) VALUES (7, '6254621063659158', 'Credit', 0);
INSERT INTO `deudas` (`id`, `number`, `type`, `deuda`) VALUES (8, '3540277358414758', 'Credit', 0);
INSERT INTO `deudas` (`id`, `number`, `type`, `deuda`) VALUES (9, '3544195834712254', 'Credit', 0);
INSERT INTO `deudas` (`id`, `number`, `type`, `deuda`) VALUES (10, '3562353412328277', 'Credit', 0);
INSERT INTO `deudas` (`id`, `number`, `type`, `deuda`) VALUES (11, '377941154739635', 'Credit', 0);
INSERT INTO `deudas` (`id`, `number`, `type`, `deuda`) VALUES (12, '6225067411483261', 'Credit', 0);
INSERT INTO `deudas` (`id`, `number`, `type`, `deuda`) VALUES (13, '3569191270336054', 'Credit', 0);
INSERT INTO `deudas` (`id`, `number`, `type`, `deuda`) VALUES (14, '6220674525815489', 'Credit', 0);
INSERT INTO `deudas` (`id`, `number`, `type`, `deuda`) VALUES (15, '4744741037451038', 'Credit', 0);
INSERT INTO `deudas` (`id`, `number`, `type`, `deuda`) VALUES (16, '4380368973378899', 'Credit', 0);
INSERT INTO `deudas` (`id`, `number`, `type`, `deuda`) VALUES (17, '5113626797107856', 'Credit', 0);
INSERT INTO `deudas` (`id`, `number`, `type`, `deuda`) VALUES (18, '6258620234092196', 'Credit', 0);
INSERT INTO `deudas` (`id`, `number`, `type`, `deuda`) VALUES (19, '347549945308755', 'Credit', 0);
INSERT INTO `deudas` (`id`, `number`, `type`, `deuda`) VALUES (20, '379187255455808', 'Credit', 0);
INSERT INTO `deudas` (`id`, `number`, `type`, `deuda`) VALUES (21, '4293032601455240', 'Credit', 0);
INSERT INTO `deudas` (`id`, `number`, `type`, `deuda`) VALUES (22, '3582999250446443', 'Credit', 0);
INSERT INTO `deudas` (`id`, `number`, `type`, `deuda`) VALUES (23, '5177227136378879', 'Credit', 0);
INSERT INTO `deudas` (`id`, `number`, `type`, `deuda`) VALUES (24, '4286229168107425', 'Credit', 0);
INSERT INTO `deudas` (`id`, `number`, `type`, `deuda`) VALUES (25, '6201020248284194', 'Credit', 0);
INSERT INTO `deudas` (`id`, `number`, `type`, `deuda`) VALUES (26, '377844129617821', 'Credit', 0);
INSERT INTO `deudas` (`id`, `number`, `type`, `deuda`) VALUES (27, '4526849118434197', 'Credit', 0);
INSERT INTO `deudas` (`id`, `number`, `type`, `deuda`) VALUES (28, '5534717431312845', 'Credit', 0);
INSERT INTO `deudas` (`id`, `number`, `type`, `deuda`) VALUES (29, '5460291941439397', 'Credit', 0);
INSERT INTO `deudas` (`id`, `number`, `type`, `deuda`) VALUES (30, '6222486987359502', 'Credit', 0);
INSERT INTO `deudas` (`id`, `number`, `type`, `deuda`) VALUES (31, '3552937551831794', 'Credit', 0);
INSERT INTO `deudas` (`id`, `number`, `type`, `deuda`) VALUES (32, '4955252418340133', 'Credit', 0);
INSERT INTO `deudas` (`id`, `number`, `type`, `deuda`) VALUES (33, '340157907902107', 'Credit', 0);
INSERT INTO `deudas` (`id`, `number`, `type`, `deuda`) VALUES (34, '6205523334355543', 'Credit', 0);
INSERT INTO `deudas` (`id`, `number`, `type`, `deuda`) VALUES (35, '3563446572483616', 'Credit', 0);
INSERT INTO `deudas` (`id`, `number`, `type`, `deuda`) VALUES (36, '3536855174531776', 'Credit', 0);
INSERT INTO `deudas` (`id`, `number`, `type`, `deuda`) VALUES (37, '5556385560239669', 'Credit', 0);
INSERT INTO `deudas` (`id`, `number`, `type`, `deuda`) VALUES (38, '5552341989067789', 'Credit', 0);
INSERT INTO `deudas` (`id`, `number`, `type`, `deuda`) VALUES (39, '370299607958063', 'Credit', 0);
INSERT INTO `deudas` (`id`, `number`, `type`, `deuda`) VALUES (40, '6287120193221873', 'Credit', 0);
INSERT INTO `deudas` (`id`, `number`, `type`, `deuda`) VALUES (41, '6229161765733243', 'Credit', 0);
INSERT INTO `deudas` (`id`, `number`, `type`, `deuda`) VALUES (42, '3557595847148012', 'Credit', 0);
INSERT INTO `deudas` (`id`, `number`, `type`, `deuda`) VALUES (43, '6258281669975213', 'Credit', 0);
INSERT INTO `deudas` (`id`, `number`, `type`, `deuda`) VALUES (44, '4480648722767107', 'Credit', 0);
INSERT INTO `deudas` (`id`, `number`, `type`, `deuda`) VALUES (45, '6221989210477281', 'Credit', 0);
INSERT INTO `deudas` (`id`, `number`, `type`, `deuda`) VALUES (46, '5089770985719872', 'Credit', 0);
INSERT INTO `deudas` (`id`, `number`, `type`, `deuda`) VALUES (47, '344049301031797', 'Credit', 0);
INSERT INTO `deudas` (`id`, `number`, `type`, `deuda`) VALUES (48, '3544471253824239', 'Credit', 0);
INSERT INTO `deudas` (`id`, `number`, `type`, `deuda`) VALUES (49, '375739030335772', 'Credit', 0);
INSERT INTO `deudas` (`id`, `number`, `type`, `deuda`) VALUES (50, '4371175609799641', 'Credit', 0);
INSERT INTO `deudas` (`id`, `number`, `type`, `deuda`) VALUES (51, '6249926993750706', 'Credit', 0);
INSERT INTO `deudas` (`id`, `number`, `type`, `deuda`) VALUES (52, '3533821797813275', 'Credit', 0);
INSERT INTO `deudas` (`id`, `number`, `type`, `deuda`) VALUES (53, '5329884835842782', 'Credit', 0);
INSERT INTO `deudas` (`id`, `number`, `type`, `deuda`) VALUES (54, '340779047754765', 'Credit', 0);
INSERT INTO `deudas` (`id`, `number`, `type`, `deuda`) VALUES (55, '6239306717945878', 'Credit', 0);
INSERT INTO `deudas` (`id`, `number`, `type`, `deuda`) VALUES (56, '4465724455073572', 'Credit', 0);
INSERT INTO `deudas` (`id`, `number`, `type`, `deuda`) VALUES (57, '372628600140626', 'Credit', 0);
INSERT INTO `deudas` (`id`, `number`, `type`, `deuda`) VALUES (58, '5539995213950868', 'Credit', 0);
INSERT INTO `deudas` (`id`, `number`, `type`, `deuda`) VALUES (59, '5037179559442619', 'Credit', 0);
INSERT INTO `deudas` (`id`, `number`, `type`, `deuda`) VALUES (60, '3571008299256327', 'Credit', 0);
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
-- Records of withdrawals
-- ----------------------------
BEGIN;
COMMIT;

-- ----------------------------
-- Triggers structure for table cards
-- ----------------------------
DROP TRIGGER IF EXISTS `insert_credit`;
delimiter ;;
CREATE TRIGGER `insert_credit` AFTER INSERT ON `cards` FOR EACH ROW IF NEW.type = 'Credit' THEN
    INSERT INTO deudas (number, type, deuda) VALUES (NEW.number, NEW.type, 0);
END IF
;;
delimiter ;

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

SET GLOBAL event_scheduler = ON;

DROP EVENT IF EXISTS `updateExpired`;
CREATE EVENT `updateExpired`
    ON SCHEDULE
        EVERY '30' SECOND STARTS '2022-11-27 00:00:00' ENDS '2023-11-27 00:00:00'
    ON COMPLETION PRESERVE
    DO UPDATE cards SET expired = 1 WHERE expiration_date <= now();

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

drop user if exists daniel@'%';
create user daniel@'%'
    identified by '1234';

grant update on table cards to daniel@'%';
grant insert on table deposits to daniel@'%';
grant insert on table transfers to daniel@'%';
grant insert on table withdrawals to daniel@'%';
grant insert on table atms to daniel@'%';
grant update on table atms to daniel@'%';

drop user if exists suadmin@'%';
create user suadmin@'%'
    identified by '1234';

GRANT ALL PRIVILEGES ON * TO suadmin@'%';

SET FOREIGN_KEY_CHECKS = 1;
