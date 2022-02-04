table! {
    msl_partners (id) {
        id -> Int4,
        name -> Text,
        locations -> Nullable<Array<Text>>,
        logo -> Nullable<Text>,
        description -> Text,
        offer -> Nullable<Text>,
        photos -> Nullable<Array<Text>>,
    }
}

table! {
    residents (id) {
        id -> Int4,
        first_name -> Text,
        last_name -> Nullable<Text>,
        dob -> Text,
        checkin -> Text,
        checkout -> Text,
        email -> Text,
        profile_photo -> Nullable<Text>,
        entry_status -> Nullable<Int4>,
    }
}

allow_tables_to_appear_in_same_query!(
    msl_partners,
    residents,
);
