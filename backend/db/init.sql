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
    ('Veld T-Box'),
    ('Veld Willy Naessens'),
    ('Veld Horta'),
    ('Veld Kartonnage Joye'),
    ('Veld Rotary Oudenaarde'),
    ('Veld Rotar Kluisbergen'),
    ('Veld Rentec'),
    ('Veld Advocatuur Diependaele');

-- Insert sample data into rounds table
INSERT INTO rounds (round_name) 
VALUES 
    ('Poule fase'),
    ('Achtste finale'),
    ('Kwartfinale'),
    ('Halve finale'),
    ('Verliezers finale'),
    ('Finale');

-- Insert sample data into groups table
INSERT INTO groups (group_name, field_id) 
VALUES 
    ('Poule 1', 1),
    ('Poule 2', 1),
    ('Poule 3', 2),
    ('Poule 4', 2),
    ('Poule 5', 3),
    ('Poule 6', 3),
    ('Poule 7', 4),
    ('Poule 8', 4),
    ('Poule 9', 5),
    ('Poule 10', 5),
    ('Poule 11', 6),
    ('Poule 12', 6),
    ('Poule 13', 7),
    ('Poule 14', 7),
    ('Poule 15', 8),
    ('Poule 16', 8);

-- Insert sample data into teams table
INSERT INTO teams (team_name, captain_name, group_id) 
VALUES 
    ('T-box en zonen', 'T-box', 1),
    ('Tom''s Balls', 'Vincent Meul', 1),
    ('Camille Cooman Fanclub', 'Camille Cooman 1', 1),
    ('Leo''s Audenaerde 1', 'Alexia Devos 1', 1),
    ('Boulettes', 'Suze Lievrouw 1', 2),
    ('Campus Marcus 1', 'Camille Ardaen 1', 2),
    ('Lemarcq design 2', 'Arne Vercruyssen 2', 2),
    ('FC Lieverbier', 'Felix Van Dycke', 2),
    ('Willy Naessens', 'Willy Naessens', 3),
    ('Fidesco', 'Filip Desmet', 3),
    ('De Gladde Ballen', 'Camille Cooman 2', 3),
    ('De Stalen Ballen', 'Axel Maertens', 3),
    ('t Is af', 'Casper Kerckvoorde', 4),
    ('Campus Marcus 2', 'Camille Ardaen 2', 4),
    ('Lemarcq design 1', 'Arne Vercruyssen 1', 4),
    ('Les amies de cochonnet', 'Marie Danneels', 4),
    ('Horta', 'Horta', 5),
    ('VSL = Vichtse Stalen Lever', 'Charlotte De Ruyck', 5),
    ('Sabrina''s', 'Yoni Vermeire', 5),
    ('Leo''s Audenaerde 2', 'Alexia Devos 2', 5),
    ('Brugge 1', 'Danielle Sabbe 1', 6),
    ('Campus Marcus 3', 'Camille Ardaen 3', 6),
    ('Abisquad', 'Arne Vercruyssen 3', 6),
    ('Petanquete voor die bloemen', 'Eveline Hove', 6),
    ('Kartonnage Joye 1', 'Kartonnage Joye 1', 7),
    ('De Kidiboules', 'Justine De Jaeger 1', 7),
    ('Ghentique', 'Charlotte Alen', 7),
    ('Snuitertjes A', 'Arnaud Acx 1', 7),
    ('Kartonnage Joye 2', 'Kartonnage Joye 2', 8),
    ('De Boules de Berlin', 'Justine De Jaeger 2', 8),
    ('Team Tuuriestar', 'Tobie Nachtergaele', 8),
    ('Snuitertjes B', 'Arnaud Acx 2', 8),
    ('The Unbeatables', 'Rotary Oudenaarde', 9),
    ('Team Pastis', 'Helene Nys', 9),
    ('Erotica', 'Charlotte Van den Storme 1', 9),
    ('De Cochonnetjes', 'Antony Desmet', 9),
    ('Brugge 2', 'Danielle Sabbe 2', 10),
    ('Margiloef', 'Nicolas Mertens', 10),
    ('De Boulekes', 'Arne Vercruyssen 4', 10),
    ('Big Ballss', 'Ben Wieme', 10),
    ('Rotary Kluisbergen', 'Rotary Kluisbergen', 11),
    ('Rotaract Mandeleie', 'Louise Van Cleemput', 11),
    ('Boule d''avant', 'Charlotte Van den Storme 2', 11),
    ('JMCE', 'Julie Brabant', 11),
    ('The Beatables', 'Charlotte Santens 1', 12),
    ('Pit Boules', 'Nina Blommaert', 12),
    ('Team Negenduust', 'Antonia Derie 2', 12),
    ('Boul et Broel', 'Charlotte Van den Storme', 12),
    ('Rentec', 'Rentec', 13),
    ('De Bouledozers', 'Kyamo De Smet', 13),
    ('Victory', 'Charlotte Van den Storme 3', 13),
    ('Bolle Babes', 'Jana Wieme', 13),
    ('DDD', 'Charlotte Santens 2', 14),
    ('RAK petanque', 'Louis Heirbrandt', 14),
    ('Team Fourchette', 'Antonia Derie 1', 14),
    ('Top G''s', 'Niels Craeye', 14),
    ('Diependaele Advocatuur', 'Advocatuur Diependaele', 15),
    ('De Powerboules Girls', 'Luca De Coninck', 15),
    ('Petanquet ze nog eens vol', 'Justine Buyle', 15),
    ('De Gastjes', 'Suze Lievrouw 2', 15),
    ('De Lustige Duifjes', 'Michèle Werbrouck', 16),
    ('Après Pétanque', 'Béatrice Santens', 16),
    ('RAKsuct', 'Emile De Leyn', 16),
    ('BOULES-ZOEKERS', 'Rune Peyskens', 16);

-- -- Insert sample data into games table
INSERT INTO games (field_id, round_id, team_1_id, team_2_id, played, start_datetime, end_datetime) 
VALUES
    (1, 1, 1, 2, FALSE, '2024-04-20 14:00:00', '2024-04-20 14:15:00'),
    (1, 1, 3, 4, FALSE, '2024-04-20 14:15:00', '2024-04-20 14:30:00'),
    (1, 1, 5, 6, FALSE, '2024-04-20 14:30:00', '2024-04-20 14:45:00'),
    (1, 1, 7, 8, FALSE, '2024-04-20 14:45:00', '2024-04-20 15:00:00'),
    (1, 1, 1, 3, FALSE, '2024-04-20 15:00:00', '2024-04-20 15:15:00'),
    (1, 1, 2, 4, FALSE, '2024-04-20 15:15:00', '2024-04-20 15:30:00'),
    (1, 1, 5, 7, FALSE, '2024-04-20 15:30:00', '2024-04-20 15:45:00'),
    (1, 1, 6, 8, FALSE, '2024-04-20 15:45:00', '2024-04-20 16:00:00'),
    (1, 1, 1, 4, FALSE, '2024-04-20 16:00:00', '2024-04-20 16:15:00'),
    (1, 1, 2, 3, FALSE, '2024-04-20 16:15:00', '2024-04-20 16:30:00'),
    (1, 1, 5, 8, FALSE, '2024-04-20 16:30:00', '2024-04-20 16:45:00'),
    (1, 1, 6, 7, FALSE, '2024-04-20 16:45:00', '2024-04-20 17:00:00'),
    (2, 1, 9, 10, FALSE, '2024-04-20 14:00:00', '2024-04-20 14:15:00'),
    (2, 1, 11, 12, FALSE, '2024-04-20 14:15:00', '2024-04-20 14:30:00'),
    (2, 1, 13, 14, FALSE, '2024-04-20 14:30:00', '2024-04-20 14:45:00'),
    (2, 1, 15, 16, FALSE, '2024-04-20 14:45:00', '2024-04-20 15:00:00'),
    (2, 1, 9, 11, FALSE, '2024-04-20 15:00:00', '2024-04-20 15:15:00'),
    (2, 1, 10, 12, FALSE, '2024-04-20 15:15:00', '2024-04-20 15:30:00'),
    (2, 1, 13, 15, FALSE, '2024-04-20 15:30:00', '2024-04-20 15:45:00'),
    (2, 1, 14, 16, FALSE, '2024-04-20 15:45:00', '2024-04-20 16:00:00'),
    (2, 1, 9, 12, FALSE, '2024-04-20 16:00:00', '2024-04-20 16:15:00'),
    (2, 1, 10, 11, FALSE, '2024-04-20 16:15:00', '2024-04-20 16:30:00'),
    (2, 1, 13, 16, FALSE, '2024-04-20 16:30:00', '2024-04-20 16:45:00'),
    (2, 1, 14, 15, FALSE, '2024-04-20 16:45:00', '2024-04-20 17:00:00'),
    (3, 1, 17, 18, FALSE, '2024-04-20 14:00:00', '2024-04-20 14:15:00'),
    (3, 1, 19, 20, FALSE, '2024-04-20 14:15:00', '2024-04-20 14:30:00'),
    (3, 1, 21, 22, FALSE, '2024-04-20 14:30:00', '2024-04-20 14:45:00'),
    (3, 1, 23, 24, FALSE, '2024-04-20 14:45:00', '2024-04-20 15:00:00'),
    (3, 1, 17, 19, FALSE, '2024-04-20 15:00:00', '2024-04-20 15:15:00'),
    (3, 1, 18, 20, FALSE, '2024-04-20 15:15:00', '2024-04-20 15:30:00'),
    (3, 1, 21, 23, FALSE, '2024-04-20 15:30:00', '2024-04-20 15:45:00'),
    (3, 1, 22, 24, FALSE, '2024-04-20 15:45:00', '2024-04-20 16:00:00'),
    (3, 1, 17, 20, FALSE, '2024-04-20 16:00:00', '2024-04-20 16:15:00'),
    (3, 1, 18, 19, FALSE, '2024-04-20 16:15:00', '2024-04-20 16:30:00'),
    (3, 1, 21, 24, FALSE, '2024-04-20 16:30:00', '2024-04-20 16:45:00'),
    (3, 1, 22, 23, FALSE, '2024-04-20 16:45:00', '2024-04-20 17:00:00'),
    (4, 1, 25, 26, FALSE, '2024-04-20 14:00:00', '2024-04-20 14:15:00'),
    (4, 1, 27, 28, FALSE, '2024-04-20 14:15:00', '2024-04-20 14:30:00'),
    (4, 1, 29, 30, FALSE, '2024-04-20 14:30:00', '2024-04-20 14:45:00'),
    (4, 1, 31, 32, FALSE, '2024-04-20 14:45:00', '2024-04-20 15:00:00'),
    (4, 1, 25, 27, FALSE, '2024-04-20 15:00:00', '2024-04-20 15:15:00'),
    (4, 1, 26, 28, FALSE, '2024-04-20 15:15:00', '2024-04-20 15:30:00'),
    (4, 1, 29, 31, FALSE, '2024-04-20 15:30:00', '2024-04-20 15:45:00'),
    (4, 1, 30, 32, FALSE, '2024-04-20 15:45:00', '2024-04-20 16:00:00'),
    (4, 1, 25, 28, FALSE, '2024-04-20 16:00:00', '2024-04-20 16:15:00'),
    (4, 1, 26, 27, FALSE, '2024-04-20 16:15:00', '2024-04-20 16:30:00'),
    (4, 1, 29, 32, FALSE, '2024-04-20 16:30:00', '2024-04-20 16:45:00'),
    (4, 1, 30, 31, FALSE, '2024-04-20 16:45:00', '2024-04-20 17:00:00'),
    (5, 1, 33, 34, FALSE, '2024-04-20 14:00:00', '2024-04-20 14:15:00'),
    (5, 1, 35, 36, FALSE, '2024-04-20 14:15:00', '2024-04-20 14:30:00'),
    (5, 1, 37, 38, FALSE, '2024-04-20 14:30:00', '2024-04-20 14:45:00'),
    (5, 1, 39, 40, FALSE, '2024-04-20 14:45:00', '2024-04-20 15:00:00'),
    (5, 1, 33, 35, FALSE, '2024-04-20 15:00:00', '2024-04-20 15:15:00'),
    (5, 1, 34, 36, FALSE, '2024-04-20 15:15:00', '2024-04-20 15:30:00'),
    (5, 1, 37, 39, FALSE, '2024-04-20 15:30:00', '2024-04-20 15:45:00'),
    (5, 1, 38, 40, FALSE, '2024-04-20 15:45:00', '2024-04-20 16:00:00'),
    (5, 1, 33, 36, FALSE, '2024-04-20 16:00:00', '2024-04-20 16:15:00'),
    (5, 1, 34, 35, FALSE, '2024-04-20 16:15:00', '2024-04-20 16:30:00'),
    (5, 1, 37, 40, FALSE, '2024-04-20 16:30:00', '2024-04-20 16:45:00'),
    (5, 1, 38, 39, FALSE, '2024-04-20 16:45:00', '2024-04-20 17:00:00'),
    (6, 1, 41, 42, FALSE, '2024-04-20 14:00:00', '2024-04-20 14:15:00'),
    (6, 1, 43, 44, FALSE, '2024-04-20 14:15:00', '2024-04-20 14:30:00'),
    (6, 1, 45, 46, FALSE, '2024-04-20 14:30:00', '2024-04-20 14:45:00'),
    (6, 1, 47, 48, FALSE, '2024-04-20 14:45:00', '2024-04-20 15:00:00'),
    (6, 1, 41, 43, FALSE, '2024-04-20 15:00:00', '2024-04-20 15:15:00'),
    (6, 1, 42, 44, FALSE, '2024-04-20 15:15:00', '2024-04-20 15:30:00'),
    (6, 1, 45, 47, FALSE, '2024-04-20 15:30:00', '2024-04-20 15:45:00'),
    (6, 1, 46, 48, FALSE, '2024-04-20 15:45:00', '2024-04-20 16:00:00'),
    (6, 1, 41, 44, FALSE, '2024-04-20 16:00:00', '2024-04-20 16:15:00'),
    (6, 1, 42, 43, FALSE, '2024-04-20 16:15:00', '2024-04-20 16:30:00'),
    (6, 1, 45, 48, FALSE, '2024-04-20 16:30:00', '2024-04-20 16:45:00'),
    (6, 1, 46, 47, FALSE, '2024-04-20 16:45:00', '2024-04-20 17:00:00'),
    (7, 1, 49, 50, FALSE, '2024-04-20 14:00:00', '2024-04-20 14:15:00'),
    (7, 1, 51, 52, FALSE, '2024-04-20 14:15:00', '2024-04-20 14:30:00'),
    (7, 1, 53, 54, FALSE, '2024-04-20 14:30:00', '2024-04-20 14:45:00'),
    (7, 1, 55, 56, FALSE, '2024-04-20 14:45:00', '2024-04-20 15:00:00'),
    (7, 1, 49, 51, FALSE, '2024-04-20 15:00:00', '2024-04-20 15:15:00'),
    (7, 1, 50, 52, FALSE, '2024-04-20 15:15:00', '2024-04-20 15:30:00'),
    (7, 1, 53, 55, FALSE, '2024-04-20 15:30:00', '2024-04-20 15:45:00'),
    (7, 1, 54, 56, FALSE, '2024-04-20 15:45:00', '2024-04-20 16:00:00'),
    (7, 1, 49, 52, FALSE, '2024-04-20 16:00:00', '2024-04-20 16:15:00'),
    (7, 1, 50, 51, FALSE, '2024-04-20 16:15:00', '2024-04-20 16:30:00'),
    (7, 1, 53, 56, FALSE, '2024-04-20 16:30:00', '2024-04-20 16:45:00'),
    (7, 1, 54, 55, FALSE, '2024-04-20 16:45:00', '2024-04-20 17:00:00'),
    (8, 1, 57, 58, FALSE, '2024-04-20 14:00:00', '2024-04-20 14:15:00'),
    (8, 1, 59, 60, FALSE, '2024-04-20 14:15:00', '2024-04-20 14:30:00'),
    (8, 1, 61, 62, FALSE, '2024-04-20 14:30:00', '2024-04-20 14:45:00'),
    (8, 1, 63, 64, FALSE, '2024-04-20 14:45:00', '2024-04-20 15:00:00'),
    (8, 1, 57, 59, FALSE, '2024-04-20 15:00:00', '2024-04-20 15:15:00'),
    (8, 1, 58, 60, FALSE, '2024-04-20 15:15:00', '2024-04-20 15:30:00'),
    (8, 1, 61, 63, FALSE, '2024-04-20 15:30:00', '2024-04-20 15:45:00'),
    (8, 1, 62, 64, FALSE, '2024-04-20 15:45:00', '2024-04-20 16:00:00'),
    (8, 1, 57, 60, FALSE, '2024-04-20 16:00:00', '2024-04-20 16:15:00'),
    (8, 1, 58, 59, FALSE, '2024-04-20 16:15:00', '2024-04-20 16:30:00'),
    (8, 1, 61, 64, FALSE, '2024-04-20 16:30:00', '2024-04-20 16:45:00'),
    (8, 1, 62, 63, FALSE, '2024-04-20 16:45:00', '2024-04-20 17:00:00');