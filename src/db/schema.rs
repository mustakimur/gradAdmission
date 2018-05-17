table! {
    basicinfo (id) {
        id -> Integer,
        first_name -> Text,
        last_name -> Text,
        nationality -> Text,
        sex -> Text,
        program -> Text,
        under_college -> Text,
        under_gpa -> Float,
        ms_college -> Nullable<Text>,
        ms_gpa -> Nullable<Float>,
        interests -> Nullable<Text>,
        decision -> Text,
    }
}
