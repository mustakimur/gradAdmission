CREATE TABLE `ApplicationsTbl` (
	`emp_id`	INTEGER NOT NULL,
	`applicant_id`	INTEGER NOT NULL,
	`name`	TEXT NOT NULL,
	`dob`	TEXT NOT NULL,
	`gender`	TEXT NOT NULL,
	`country`	TEXT NOT NULL,
	`program`	TEXT NOT NULL,
	`degree`	TEXT NOT NULL,
	`interests`	TEXT NOT NULL,
	`ug_university`	TEXT NOT NULL,
	`ug_major`	TEXT NOT NULL,
	`ug_degree`	TEXT NOT NULL,
	`ug_gpa`	REAL NOT NULL,
	`grad_university`	TEXT NOT NULL,
	`grad_major`	TEXT NOT NULL,
	`grad_degree`	TEXT NOT NULL,
	`grad_gpa`	REAL NOT NULL,
	`toefl_ielts`	INTEGER NOT NULL,
	`gre_verb`	INTEGER NOT NULL,
	`gre_quanti`	INTEGER NOT NULL,
	`gre_combined`	INTEGER NOT NULL,
	`decision`	TEXT NOT NULL,
	`advisor`	TEXT NOT NULL,
	`assistantship`	TEXT NOT NULL,
	`fte`	REAL NOT NULL,
	`yearly_amount`	INTEGER NOT NULL,
	PRIMARY KEY(`applicant_id`)
);

CREATE TABLE `CommentsTbl` (
	`comment_id`	INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT UNIQUE,
	`emp_id`	INTEGER NOT NULL,
	`commenter`	TEXT NOT NULL,
	`opinion`	TEXT NOT NULL
);

CREATE TABLE `UsersTbl` (
	`user_name`	TEXT NOT NULL,
	`role`	TEXT NOT NULL,
	`password`	TEXT NOT NULL,
	PRIMARY KEY(`user_name`)
);