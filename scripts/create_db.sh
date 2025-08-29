

create_db(){
    # Create USER and DATABASE Dominote to manage with your own user
sudo -u postgres psql -c "CREATE USER ${LOGNAME} WITH SUPERUSER PASSWORD 'domipswd';"
sudo -u postgres psql -c "CREATE DATABASE dominote"
create_players_table
}

create_players_table(){
    # Create table for Dominote
    sudo -u postgres psql -d dominote -c "CREATE TABLE IF NOT EXISTS players (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(), 
    name VARCHAR(100) UNIQUE NOT NULL, 
    score INTEGER DEFAULT 0,
    wins INTEGER DEFAULT 0,
    games_played INTEGER DEFAULT 0,
    CONSTRAINT score_check CHECK (score >= 0),
    CONSTRAINT games_played_check CHECK (games_played >= 0),
    CONSTRAINT wins_check CHECK (wins >= 0),
    CONSTRAINT chk_wins_vs_games CHECK (games_played >= wins)
    )
"
}

create_db
