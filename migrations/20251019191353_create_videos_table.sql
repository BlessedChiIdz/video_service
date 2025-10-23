CREATE TABLE videos (
    id SERIAL PRIMARY KEY,
    title VARCHAR(255) NOT NULL,
    description TEXT,
    file_path VARCHAR(500) NOT NULL,
    duration INTEGER,
    file_size BIGINT,
    creation_user INTEGER NOT NULL,
    created_at TIMESTAMP DEFAULT NOW(),
    views_count BIGINT DEFAULT 0,

    CONSTRAINT fk_creation_user
        FOREIGN KEY (creation_user)
            REFERENCES users(id)
            ON DELETE CASCADE,

    CONSTRAINT positive_duration CHECK (duration IS NULL OR duration > 0),
    CONSTRAINT positive_file_size CHECK (file_size IS NULL OR file_size > 0)
);

CREATE INDEX idx_videos_creation_user ON videos(creation_user);
CREATE INDEX idx_videos_created_at ON videos(created_at DESC);
CREATE INDEX idx_videos_title ON videos USING gin(to_tsvector('english', title));