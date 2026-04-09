-- 時間割テーブル
CREATE TABLE IF NOT EXISTS schedules (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    day_of_week INTEGER NOT NULL, -- 0:月, 1:火...
    period INTEGER NOT NULL,      -- 1限, 2限...
    subject TEXT NOT NULL,
    room TEXT
);

-- 課題テーブル
CREATE TABLE IF NOT EXISTS tasks (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    title TEXT NOT NULL,
    description TEXT,
    deadline DATETIME NOT NULL,
    is_completed BOOLEAN NOT NULL DEFAULT 0
);
