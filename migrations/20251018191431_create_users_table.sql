CREATE TYPE user_role AS ENUM ('user', 'admin', 'moderator');

CREATE TABLE Users (
   id SERIAL PRIMARY KEY,
   username VARCHAR(50) UNIQUE NOT NULL,
   email VARCHAR(100) UNIQUE NOT NULL,
   password_hash VARCHAR(255) NOT NULL,
   created_at TIMESTAMP DEFAULT NOW(),
   updated_at TIMESTAMP DEFAULT NOW(),
   is_active BOOLEAN DEFAULT TRUE,
   role user_role DEFAULT 'user'
);