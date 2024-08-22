// @generated automatically by Diesel CLI.


diesel::table! {
    categories (id) {
        id -> Int8,
        created_at -> Nullable<Timestamptz>,
        updated_at -> Nullable<Timestamptz>,
        category_uuid -> Uuid,
        catogery_name -> Text,
        imgurl -> Text,
        order -> Nullable<Int8>,
    }
}

diesel::table! {
    permissions (id) {
        id -> Int8,
        created_at -> Nullable<Timestamptz>,
        updated_at -> Nullable<Timestamptz>,
        permission1 -> Text,
    }
}



diesel::table! {
    products (id) {
        id -> Int8,
        created_at -> Nullable<Timestamptz>,
        updated_at -> Nullable<Timestamptz>,
        product_uuid -> Uuid,
        product_name -> Text,
        category_id -> Nullable<Int8>,
        description -> Text,
        family -> Nullable<Text>,
        variant -> Text,
        discount -> Int8,
    }
}



diesel::table! {
    role_entities (id) {
        id -> Int8,
        created_at -> Nullable<Timestamptz>,
        updated_at -> Nullable<Timestamptz>,
        entity_id -> Nullable<Text>,
        entity_name -> Nullable<Text>,
        role_id -> Int8,
        logger_id -> Uuid,
    }
}

diesel::table! {
    roles (id) {
        id -> Int8,
        created_at -> Nullable<Timestamptz>,
        updated_at -> Nullable<Timestamptz>,
        name -> Text,
    }
}



diesel::table! {
    users (id) {
        id -> Int8,
        created_at -> Nullable<Timestamptz>,
        updated_at -> Nullable<Timestamptz>,
        user_id -> Uuid,
        name -> Text,
        first_name -> Text,
        last_name -> Text,
        email -> Text,
        password -> Text,
        phone_number -> Nullable<Text>,
        isemail_verfied -> Nullable<Bool>,
        is_phone_verfied -> Nullable<Bool>,
        is_blocked -> Nullable<Bool>,
        is_active -> Nullable<Bool>,
    }
}

diesel::table! {
    product_attributes (id) {
        id -> Int8,
        created_at -> Nullable<Timestamptz>,
        updated_at -> Nullable<Timestamptz>,
        product_attrb_uuid -> Uuid,
        product_id -> Nullable<Int8>,
        image_one -> Nullable<Text>,
        img_two -> Nullable<Text>,
        img_three -> Nullable<Text>,
    }
}

diesel::joinable!(products -> categories (category_id));
diesel::joinable!(product_attributes -> products (product_id));

diesel::joinable!(role_entities -> roles (role_id));
//diesel::enable_multi_table_joins!(users, role_entities);






diesel::allow_tables_to_appear_in_same_query!(    
    categories,    
    permissions,    
    products,
    product_attributes,    
    role_entities,
    roles,    
    users,
);







