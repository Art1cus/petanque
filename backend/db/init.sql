DO $$
BEGIN
    IF EXISTS (SELECT 1 FROM pg_database WHERE datname='petanque') THEN
        DROP DATABASE petanque;
    END IF;
END $$;

-- Create the petanque database
CREATE DATABASE petanque;

-- Connect to the petanque database
\c petanque;

-- Create the fields table
CREATE TABLE fields (
    field_id SERIAL PRIMARY KEY,
    field_name VARCHAR(255) NOT NULL
);

-- Create the rounds table
CREATE TABLE rounds (
    round_id SERIAL PRIMARY KEY,
    round_name VARCHAR(255) NOT NULL
);

-- Create the groups table
CREATE TABLE groups (
    group_id SERIAL PRIMARY KEY,
    group_name VARCHAR(255) NOT NULL,
    field_id INT REFERENCES fields(field_id)
);

-- Create the teams table
CREATE TABLE teams (
    team_id SERIAL PRIMARY KEY,
    team_name VARCHAR(255) NOT NULL,
    captain_name VARCHAR(255),
    contact_email VARCHAR(255),
    group_id INT REFERENCES groups(group_id)
);

-- Create the games table
CREATE TABLE games (
    game_id SERIAL PRIMARY KEY,
    field_id INT REFERENCES fields(field_id),
    round_id INT REFERENCES rounds(round_id),
    team_1_id INT REFERENCES teams(team_id),
    team_2_id INT REFERENCES teams(team_id),
    played BOOLEAN DEFAULT FALSE,
    start_datetime TIMESTAMP,
    end_datetime TIMESTAMP
);

-- Create the game_results table
CREATE TABLE game_results (
    result_id SERIAL PRIMARY KEY,
    game_id INT REFERENCES games(game_id),
    team_id INT REFERENCES teams(team_id),
    score INT NOT NULL,
    CONSTRAINT unique_game_team_constraint UNIQUE (game_id, team_id)
);

-- Insert sample data into fields table
INSERT INTO fields (field_name) 
VALUES 
    ('Field 1'),
    ('Field 2'),
    ('Field 3'),
    ('Field 4');

-- Insert sample data into rounds table
INSERT INTO rounds (round_name) 
VALUES 
    ('Round 1'),
    ('Round 2'),
    ('Round 3');

-- Insert sample data into groups table
INSERT INTO groups (group_name, field_id) 
VALUES 
    ('Group 1', 1),
    ('Group 2', 2),
    ('Group 3', 3),
    ('Group 4', 4);

-- Insert sample data into teams table
INSERT INTO teams (team_name, captain_name, contact_email, group_id) 
VALUES 
    ('Team 1', 'Captain 1', 'captain1@example.com', 1),
    ('Team 2', 'Captain 2', 'captain2@example.com', 1),
    ('Team 3', 'Captain 3', 'captain3@example.com', 1),
    ('Team 4', 'Captain 4', 'captain4@example.com', 1),
    ('Team 5', 'Captain 5', 'captain5@example.com', 2),
    ('Team 6', 'Captain 6', 'captain6@example.com', 2),
    ('Team 7', 'Captain 7', 'captain7@example.com', 2),
    ('Team 8', 'Captain 8', 'captain8@example.com', 2),
    ('Team 9', 'Captain 9', 'captain9@example.com', 3),
    ('Team 10', 'Captain 10', 'captain10@example.com', 3),
    ('Team 11', 'Captain 11', 'captain11@example.com', 3),
    ('Team 12', 'Captain 12', 'captain12@example.com', 3),
    ('Team 13', 'Captain 13', 'captain13@example.com', 4),
    ('Team 14', 'Captain 14', 'captain14@example.com', 4),
    ('Team 15', 'Captain 15', 'captain15@example.com', 4),
    ('Team 16', 'Captain 16', 'captain16@example.com', 4);

-- Insert sample data into games table
INSERT INTO games (field_id, round_id, team_1_id, team_2_id, played, start_datetime, end_datetime) 
SELECT 
    g.field_id,
    r.round_id,
    t1.team_id,
    t2.team_id,
    FALSE,
    '2022-01-01 10:00:00',
    '2022-01-01 12:00:00'
FROM 
    groups g
CROSS JOIN 
    rounds r
JOIN 
    teams t1 ON t1.group_id = g.group_id
JOIN 
    teams t2 ON t2.group_id = g.group_id AND t1.team_id < t2.team_id;