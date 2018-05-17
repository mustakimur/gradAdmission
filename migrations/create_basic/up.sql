CREATE TABLE `basicinfo` (
	`id`	INTEGER NOT NULL,
	`first_name`	TEXT NOT NULL,
	`last_name`	TEXT NOT NULL,
	`nationality`	TEXT NOT NULL,
	`sex`		TEXT NOT NULL,
	`program`	TEXT NOT NULL,
	`under_college`	TEXT NOT NULL,
	`under_gpa`	REAL NOT NULL,
	`ms_college`	TEXT,
	`ms_gpa`	REAL,
	`interests`	TEXT,
	`decision`	TEXT NOT NULL, 
	PRIMARY KEY(`id`)
);
