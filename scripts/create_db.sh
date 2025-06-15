

create_db(){
    # Create USER and DATABASE Dominote to manage with your own user
sudo -u postgres psql -c "CREATE USER ${LOGNAME} WITH SUPERUSER;"
sudo -u postgres psql -c "CREATE DATABASE dominote"
create_table
}


create_table(){
    # Create table for Dominote
    sudo -u postgres psql -d dominote -c "CREATE TABLE IF NOT EXISTS players (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(), 
    name VARCHAR(100), 
    score INTEGER,
    games_played INTEGER);"
}


create_db
