-- migrations/20240117191858_tuesday_17_Jan_2024

CREATE TABLE IF NOT EXISTS country (
    name VARCHAR(255) PRIMARY KEY,
    continent VARCHAR(255) NOT NULL
);

CREATE TABLE IF NOT EXISTS company (
    id UUID PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    description TEXT NOT NULL,
    country_name VARCHAR(255) NOT NULL,
    created_at TIMESTAMPTZ,
    updated_at TIMESTAMPTZ,
    deleted_at TIMESTAMPTZ,
    FOREIGN KEY (country_name) REFERENCES country(name)
);

INSERT INTO country (name, continent) VALUES
('Argentina', 'South America'),
('Germany', 'Europe');

INSERT INTO company (id, name, description, country_name, created_at, updated_at, deleted_at) VALUES
('d0327d90-091d-40ea-bc4b-2072a03aaf62', 'Company A', 'Description A', 'Argentina', '2022-01-01', '2022-01-02', NULL),
('fd0f6a3e-98d0-4473-9c58-3d44893bcbda', 'Company B', 'Description B', 'Germany', '2022-01-03', '2022-01-04', NULL),
('42c04a90-5349-4d0e-8f1f-ded4be6d1590', 'Company C', 'Description C', 'Argentina', '2022-01-05', '2022-01-06', NULL),
('a827fe8b-32e9-43a4-a4b8-1d9448a7fcf9', 'Company D', 'Description D', 'Germany', '2022-01-07', '2022-01-08', NULL);
