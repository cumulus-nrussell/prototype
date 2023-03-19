CREATE TABLE game_challenges (
    id uuid default gen_random_uuid() primary key not null,
    challenger_uid text references users (uid) not null,
    game_type text not null,
    ranked boolean not null,
    public boolean not null,
    tournament_queen_rule boolean not null,
    expiration_time timestamp with time zone not null
)
