CREATE TABLE `atm_machine` (
  `ATM_id` int NOT NULL AUTO_INCREMENT,
  `ATM_Name` varchar(10) NOT NULL,
  `ATM_Add` varchar(100) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci DEFAULT NULL,
  `ATM_Bankname` varchar(15) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci DEFAULT NULL,
  `ATM_money` double NOT NULL,
  PRIMARY KEY (`ATM_id`)
) ENGINE=InnoDB AUTO_INCREMENT=6 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci;

CREATE TABLE `card` (
  `Card_No` varchar(16) NOT NULL COMMENT 'Nomber of the card',
  `Card_Bankname` varchar(15) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL COMMENT 'Name bank',
  `Card_CVV` int NOT NULL,
  `Card_ExpiryDate` date NOT NULL,
  `Card_Balance` double NOT NULL COMMENT 'Money',
  `Card_Type` varchar(10) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL COMMENT 'Credit or Debit',
  `Crad_status` bit(1) DEFAULT NULL COMMENT 'Si esta bloqueada o no',
  PRIMARY KEY (`Card_No`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci;

CREATE TABLE `customer` (
  `C_id` int DEFAULT NULL,
  `F_name` varchar(15) DEFAULT NULL COMMENT 'First name',
  `M_name` varchar(15) DEFAULT NULL COMMENT 'Middle name'
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci;

CREATE TABLE `deposit` (
  `id_deposit` int NOT NULL,
  `amount` double DEFAULT NULL COMMENT 'cantidad que deposito',
  `date` datetime DEFAULT NULL,
  PRIMARY KEY (`id_deposit`),
  CONSTRAINT `deposit_FK` FOREIGN KEY (`id_deposit`) REFERENCES `atm_machine` (`ATM_id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci;

CREATE TABLE `transfer` (
  `id_tranfer` int NOT NULL AUTO_INCREMENT,
  `Shipping` varchar(16) NOT NULL COMMENT 'envio el dinero',
  `received` varchar(16) NOT NULL COMMENT 'recibio el dinero',
  `date` datetime DEFAULT NULL,
  `amount` double DEFAULT NULL COMMENT 'cantidad que se envio',
  PRIMARY KEY (`id_tranfer`)
) ENGINE=InnoDB AUTO_INCREMENT=6 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci;

CREATE TABLE `withdrawal_money` (
  `id_withdrawal` int NOT NULL,
  `amount` double DEFAULT NULL,
  `date` date DEFAULT NULL,
  PRIMARY KEY (`id_withdrawal`),
  CONSTRAINT `Withdrawal_money_FK` FOREIGN KEY (`id_withdrawal`) REFERENCES `atm_machine` (`ATM_id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci;



insert into atm_machine (atm_id, atm_name, ATM_Add, ATM_Bankname, ATM_money)
values (1,'S_064', 'Oso 81, Col del Valle Centro, Benito Juárez, 03100 Ciudad de México, CDMX', 'Santander', 200000.0),
insert into atm_machine (atm_id, atm_name, ATM_Add, ATM_Bankname)
values (2,'S_067', 'Av. Insurgentes Sur 1883, Guadalupe Inn, Álvaro Obregón, CDMX', 'Santander', 152100.0);
insert into atm_machine (atm_id, atm_name, ATM_Add, ATM_Bankname)
values (3,'BB_021', 'Av. de los Insurgentes Sur 1323, Insurgentes Mixcoac, Benito Juárez, 03920 Ciudad de México, CDMX
', 'BBVA', 123010.0);
insert into atm_machine (atm_id, atm_name, ATM_Add, ATM_Bankname)
values (4,'BB_156', 'Av. Revolución 1579, San Ángel, Álvaro Obregón, 01000 Ciudad de México, CDMX
', 'BBVA',220000.0 );
insert into atm_machine (atm_id, atm_name, ATM_Add, ATM_Bankname)
values (5,'CB_102', 'Felipe Carrillo Puerto 3, Coyoacán, 04100 Ciudad de México, CDMX
', 'Citibanamex',190000.0 );

insert into card (card_No, Card_Bankname, Card_CVV, Card_ExpiryDate, Card_balance, Card_type, Card_status)
values ('1426045781603457', 'Santander', 423, '2025-11-11', 45215.0, 'Debit', 1);
insert into card (card_No, Card_Bankname, Card_CVV, Card_ExpiryDate, Card_balance, Card_type, Card_status)
values ('3248904237510568', 'Santander', 124, '2024-01-26', 2641.0, 'Debit', 1);
insert into card (card_No, Card_Bankname, Card_CVV, Card_ExpiryDate, Card_balance, Card_type, Card_status)
values ('4578015464893204', 'BBVA', 130, '2025-06-23', 17590.0, 'Credit', 1);
insert into card (card_No, Card_Bankname, Card_CVV, Card_ExpiryDate, Card_balance, Card_type, Card_status)
values ('7598150468901754', 'BBVA', 457, '2025-12-05', 7546.0, 'Credit', 1);
insert into card (card_No, Card_Bankname, Card_CVV, Card_ExpiryDate, Card_balance, Card_type, Card_status)
values ('8105460479831056', 'Citibanamex', 684, '2023-03-12', 24321.0, 'Debit', 1);

insert into customer(C_id, F_name, M_name)
values (1, 'Arnulfo', 'Carrera');
insert into customer(C_id, F_name, M_name)
values (2, 'Ana', 'Armira');
insert into customer(C_id, F_name, M_name)
values (3, 'María', 'Vásquez');
insert into customer(C_id, F_name, M_name)
values (4, 'Edgar', 'Culajay');
insert into customer(C_id, F_name, M_name)
values (5, 'Lilian', 'Rodríguez');

insert into deposit(id_deposit, amount, date)
values (1, 100, '2022-10-13 15:40:42');
insert into deposit(id_deposit, amount, date)
values (2, 250, '2022-10-14 10:42:11');
insert into deposit(id_deposit, amount, date)
values (3, 200, '2022-10-14 20:01:46');
insert into deposit(id_deposit, amount, date)
values (4, 1000, '2022-11-02 12:16:55');
insert into deposit(id_deposit, amount, date)
values (5, 450, '2022-11-03 13:23:02');

insert into transfer(id_transfer, Shipping, received, date, amount)
values (1, '1426045781603457', '4578015464893204', '2022-10-13 10:15:26', 1540.0);
insert into transfer(id_transfer, Shipping, received, date, amount)
values (2, '3248904237510568', '7598150468901754', '2022-10-13 23:16:56', 450.0);
insert into transfer(id_transfer, Shipping, received, date, amount)
values (3, '4578015464893204', '8105460479831056', '2022-10-29 09:46:11', 2600.0);
insert into transfer(id_transfer, Shipping, received, date, amount)
values (4, '7598150468901754', '1426045781603457', '2022-11-01 13:52:46', 5000.0);
insert into transfer(id_transfer, Shipping, received, date, amount)
values (5, '1426045781603457', '3248904237510568', '2022-11-03 20:16:10', 500.0);

insert into withdrawal_money(id_withdrawal, amount, "date")
VALUES (1, 1540, '2022-10-01');
insert into withdrawal_money(id_withdrawal, amount, "date")
VALUES (2, 540, '2022-10-12');
insert into withdrawal_money(id_withdrawal, amount, "date")
VALUES (3, 200, '2022-10-02');
insert into withdrawal_money(id_withdrawal, amount, "date")
VALUES (4, 1000, '2022-10-25');
insert into withdrawal_money(id_withdrawal, amount, "date")
VALUES (5, 2500, '2022-11-14');