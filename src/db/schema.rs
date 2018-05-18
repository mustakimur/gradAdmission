table! {
    ApplicationsTbl (EmpID) {
        EmpID -> Integer,
        ApplicantID -> Integer,
        Name -> Text,
        DOB -> Text,
        Gender -> Text,
        Country -> Text,
        Program -> Text,
        Degree -> Text,
        Interests -> Text,
        UG_University -> Integer,
        UG_Major -> Text,
        UG_Degree -> Text,
        UG_GPA -> Float,
        Grad_University -> Text,
        Grad_Major -> Text,
        Grad_Degree -> Text,
        Grad_GPA -> Float,
        TOEFL_IELTS -> Integer,
        GRE_Verb -> Integer,
        GRE_Quanti -> Integer,
        GRE_Combined -> Integer,
        Decision -> Text,
        Advisor -> Text,
        Assistantship -> Integer,
        FTE -> Float,
        YearlyAmount -> Integer,
    }
}

table! {
    CommentsTbl (CommentId) {
        CommentId -> Integer,
        EmpID -> Integer,
        Commenter -> Text,
        Opinion -> Text,
    }
}

table! {
    UsersTbl (Username) {
        Username -> Text,
        Role -> Text,
        Password -> Text,
    }
}

allow_tables_to_appear_in_same_query!(
    ApplicationsTbl,
    CommentsTbl,
    UsersTbl,
);
