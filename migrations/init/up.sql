CREATE TABLE `applications_tbl` (
	`emp_id`	INTEGER NOT NULL,
	`applicant_id`	INTEGER NOT NULL,
	`name`	TEXT NOT NULL,
	`dob`	TEXT NOT NULL,
	`gender`	TEXT NOT NULL,
	`country`	TEXT NOT NULL,
	`program`	TEXT NOT NULL,
	`degree`	TEXT NOT NULL,
	`term`	TEXT NOT NULL,
	`interests`	TEXT NOT NULL,
	`ug_university`	TEXT NOT NULL,
	`ug_major`	TEXT NOT NULL,
	`ug_degree`	TEXT NOT NULL,
	`ug_gpa`	REAL NOT NULL,
	`grad_university`	TEXT NOT NULL,
	`grad_major`	TEXT NOT NULL,
	`grad_degree`	TEXT NOT NULL,
	`grad_gpa`	REAL NOT NULL,
	`toefl_ielts`	TEXT NOT NULL,
	`gre`	TEXT NOT NULL,
	`decision`	TEXT NOT NULL,
	`advisor`	TEXT NOT NULL,
	`assistantship`	TEXT NOT NULL,
	`fte`	REAL NOT NULL,
	`yearly_amount`	INTEGER NOT NULL,
	PRIMARY KEY(`applicant_id`)
);

CREATE TABLE `comments_tbl` (
	`comment_id`	INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT UNIQUE,
	`applicant_id`	INTEGER NOT NULL,
	`commenter`	TEXT NOT NULL,
	`opinion`	TEXT NOT NULL,
	`when`	TEXT NOT NULL
);

CREATE TABLE `users_tbl` (
	`user_name`	TEXT NOT NULL,
	`role`	TEXT NOT NULL,
	`salt` TEXT NOT NULL,
	`password`	TEXT NOT NULL,
	PRIMARY KEY(`user_name`)
);

INSERT INTO `users_tbl`(`user_name`,`role`,`salt`,`password`) VALUES ('Admin','Sys','wp5vgXrE9smF3JCl','F4040E708D624EA0400D7E05C5973BD29C95B035C85DDCBEA399DAC358923827');
