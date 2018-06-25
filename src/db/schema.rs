table! {
    applications_tbl (applicant_id) {
        emp_id -> Integer,
        applicant_id -> Integer,
        name -> Text,
        dob -> Text,
        gender -> Text,
        country -> Text,
        program -> Text,
        degree -> Text,
        term -> Text,
        interests -> Text,
        ug_university -> Text,
        ug_major -> Text,
        ug_degree -> Text,
        ug_gpa -> Double,
        grad_university -> Text,
        grad_major -> Text,
        grad_degree -> Text,
        grad_gpa -> Double,
        toefl_ielts -> Text,
        gre -> Text,
        decision -> Text,
        advisor -> Text,
        assistantship -> Text,
        fte -> Double,
        yearly_amount -> Integer,
    }
}

table! {
    comments_tbl (comment_id) {
        comment_id -> Nullable<Integer>,
        applicant_id -> Integer,
        commenter -> Text,
        opinion -> Text,
        when -> Text,
    }
}

table! {
    users_tbl (user_name) {
        user_name -> Text,
        role -> Text,
        salt -> Text,
        password -> Text,
    }
}

allow_tables_to_appear_in_same_query!(applications_tbl, comments_tbl, users_tbl,);
