CREATE TABLE tasks (
    id SERIAL PRIMARY KEY,
    title VARCHAR(255) NOT NULL,
    completed BOOLEAN NOT NULL DEFAULT false
);
CREATE USER app_user WITH PASSWORD 'app_password';
CREATE DATABASE task_manager OWNER app_user;
GRANT ALL PRIVILEGES ON DATABASE task_manager TO app_user;