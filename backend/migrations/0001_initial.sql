CREATE TABLE IF NOT EXISTS Users (
    uuid uuid PRIMARY KEY NOT NULL,
    username VARCHAR(255) NOT NULL,
    email VARCHAR(255) NOT NULL,
    created_at VARCHAR(255) DEFAULT CURRENT_TIMESTAMP,
    updated_at VARCHAR(255) DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS Documents (
    uuid uuid PRIMARY KEY NOT NULL,
    user_uuid uuid NOT NULL,
    title VARCHAR(255) NOT NULL,
    content TEXT NOT NULL,
    created_at VARCHAR(255) DEFAULT CURRENT_TIMESTAMP,
    updated_at VARCHAR(255) DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT FK_user_document FOREIGN KEY(user_uuid)
        REFERENCES Users(uuid)
);

CREATE TABLE IF NOT EXISTS UserSessions (
    uuid uuid PRIMARY KEY NOT NULL,
    user_uuid uuid REFERENCES Users(uuid) NOT NULL,
    created_at VARCHAR(255) DEFAULT CURRENT_TIMESTAMP,
    expires_at VARCHAR(255) DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT FK_user_session FOREIGN KEY(user_uuid)
        REFERENCES Users(uuid)
);
