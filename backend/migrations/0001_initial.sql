CREATE TABLE IF NOT EXISTS Users (
    uuid uuid PRIMARY KEY,
    username VARCHAR(255) NOT NULL,
    email VARCHAR(255) NOT NULL,
    created_at VARCHAR(255) DEFAULT CURRENT_TIMESTAMP,
    updated_at VARCHAR(255) DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS Documents (
    uuid uuid PRIMARY KEY,
    user_uuid uuid,
    title VARCHAR(255) NOT NULL,
    content TEXT NOT NULL,
    created_at VARCHAR(255) DEFAULT CURRENT_TIMESTAMP,
    updated_at VARCHAR(255) DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT FK_user_document FOREIGN KEY(user_uuid)
        REFERENCES Users(uuid)
);
