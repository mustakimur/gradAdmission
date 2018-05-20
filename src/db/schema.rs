table! {
    ApplicationsTbl (applicant_id) {
        emp_id -> Integer,
        applicant_id -> Integer,
        name -> Text,
        dob -> Text,
        gender -> Text,
        country -> Text,
        program -> Text,
        degree -> Text,
        interests -> Text,
        ug_university -> Text,
        ug_major -> Text,
        ug_degree -> Text,
        ug_gpa -> Float,
        grad_university -> Text,
        grad_major -> Text,
        grad_degree -> Text,
        grad_gpa -> Float,
        toefl_ielts -> Integer,
        gre_verb -> Integer,
        gre_quanti -> Integer,
        gre_combined -> Integer,
        decision -> Text,
        advisor -> Text,
        assistantship -> Text,
        fte -> Float,
        yearly_amount -> Integer,
    }
}

table! {
    CommentsTbl (comment_id) {
        comment_id -> Integer,
        emp_id -> Integer,
        commenter -> Text,
        opinion -> Text,
    }
}

table! {
    UsersTbl (user_name) {
        user_name -> Text,
        role -> Text,
        password -> Text,
    }
}

allow_tables_to_appear_in_same_query!(
    ApplicationsTbl,
    CommentsTbl,
    UsersTbl,
);
