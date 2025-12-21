IF DB_ID('db') IS NULL
BEGIN
    CREATE DATABASE db;
END
GO

USE db;
GO

CREATE TABLE roles (
    id INT IDENTITY PRIMARY KEY,
    name NVARCHAR(50) NOT NULL UNIQUE,
    description NVARCHAR(255)
);

INSERT INTO roles (name, description)
VALUES 
    ('admin', 'Администратор'),
    ('user', 'Пользователь'),
    ('moderator', 'Модератор'),
    ('premium', 'Платный пользователь');

CREATE TABLE picture_type (
    id INT IDENTITY PRIMARY KEY,
    name NVARCHAR(50) NOT NULL UNIQUE
);

INSERT INTO picture_type (name)
VALUES ('quest'), ('item'), ('map'), ('avatar'), ('marker');

CREATE TABLE picture (
    id INT IDENTITY PRIMARY KEY,
    url NVARCHAR(500) NOT NULL,
    type_id INT NOT NULL,
    FOREIGN KEY (type_id) REFERENCES picture_type(id)
);

CREATE TABLE trader (
    id INT IDENTITY PRIMARY KEY,
    name NVARCHAR(100) NOT NULL,
    description NVARCHAR(500),
    picture_id INT,
    FOREIGN KEY (picture_id) REFERENCES picture(id)
);

CREATE TABLE item (
    id INT IDENTITY PRIMARY KEY,
    name NVARCHAR(100) NOT NULL,
    description NVARCHAR(500),
    picture_id INT,
    FOREIGN KEY (picture_id) REFERENCES picture(id)
);

CREATE TABLE quest (
    id INT IDENTITY PRIMARY KEY,
    name NVARCHAR(150) NOT NULL,
    description NVARCHAR(MAX),
    picture_id INT,
    trader_id INT,
    required_level INT NOT NULL,
    changed_at DATETIME2 DEFAULT SYSDATETIME(),
    FOREIGN KEY (picture_id) REFERENCES picture(id),
    FOREIGN KEY (trader_id) REFERENCES trader(id)
);

CREATE TABLE quest_reward (
    id INT IDENTITY PRIMARY KEY,
    quest_id INT NOT NULL,
    type NVARCHAR(50) NOT NULL,
    item_id INT,
    amount INT DEFAULT 1,
    reputation_amount INT,
    trader_id INT,
    FOREIGN KEY (quest_id) REFERENCES quest(id),
    FOREIGN KEY (item_id) REFERENCES item(id),
    FOREIGN KEY (trader_id) REFERENCES trader(id)
);

CREATE TABLE users (
    id INT IDENTITY PRIMARY KEY,
    username NVARCHAR(100) NOT NULL UNIQUE,
    password NVARCHAR(255) NOT NULL,
    email NVARCHAR(150),
    avatar_id INT,
    role_id INT,
    registred_at DATETIME2 DEFAULT SYSDATETIME(),
    FOREIGN KEY (avatar_id) REFERENCES picture(id),
    FOREIGN KEY (role_id) REFERENCES roles(id)
);

CREATE TABLE user_oauth (
    id INT IDENTITY PRIMARY KEY,
    user_id INT NOT NULL,
    provider NVARCHAR(50) NOT NULL,
    provider_user_id NVARCHAR(100) NOT NULL,
    FOREIGN KEY (user_id) REFERENCES users(id)
);

CREATE TABLE map (
    id INT IDENTITY PRIMARY KEY,
    name NVARCHAR(100) NOT NULL,
    description NVARCHAR(500),
    picture_id INT,
    difficulty INT,
    FOREIGN KEY (picture_id) REFERENCES picture(id)
);

CREATE TABLE comment (
    id INT IDENTITY PRIMARY KEY,
    text NVARCHAR(MAX) NOT NULL,
    author_id INT NOT NULL,
    rating INT,
    quest_id INT NOT NULL,
    changed_at DATETIME2 DEFAULT SYSDATETIME(),
    FOREIGN KEY (author_id) REFERENCES users(id),
    FOREIGN KEY (quest_id) REFERENCES quest(id)
);

CREATE TABLE map_marker (
    id INT IDENTITY PRIMARY KEY,
    map_id INT NOT NULL,
    type NVARCHAR(50),
    description NVARCHAR(500),
    access_rule NVARCHAR(100),
    picture_id INT,
    x FLOAT NOT NULL,
    y FLOAT NOT NULL,
    FOREIGN KEY (map_id) REFERENCES map(id),
    FOREIGN KEY (picture_id) REFERENCES picture(id)
);

CREATE TABLE quest_ref (
    quest_id INT NOT NULL,
    required_quest_id INT NOT NULL,
    type NVARCHAR(50),
    PRIMARY KEY (quest_id, required_quest_id),
    FOREIGN KEY (quest_id) REFERENCES quest(id),
    FOREIGN KEY (required_quest_id) REFERENCES quest(id)
);

CREATE TABLE user_complete_quest (
    user_id INT NOT NULL,
    quest_id INT NOT NULL,
    PRIMARY KEY (user_id, quest_id),
    FOREIGN KEY (user_id) REFERENCES users(id),
    FOREIGN KEY (quest_id) REFERENCES quest(id)
);

CREATE TABLE quest_map (
    quest_id INT NOT NULL,
    map_id INT NOT NULL,
    PRIMARY KEY (quest_id, map_id),
    FOREIGN KEY (quest_id) REFERENCES quest(id),
    FOREIGN KEY (map_id) REFERENCES map(id)
);
