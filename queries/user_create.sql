INSERT INTO users(login)
VALUES ($1)
    RETURNING $table_fields;
