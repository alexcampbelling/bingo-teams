Todo
- [x] 1 - read in list of usernames
- [x] 2 - check they all can be called with temple osrs api first
- [x] 3 - decide good stats to check for
- [x] 4 - create data structures for each player / teams
- [x] 5 - decide weighting for player stats to score players
    - [x] 5.1 - correctly calculate a average score
- [x] 6 - check these scores all make sense, allow for manual override for outliers
    - [x] 6.1 - find players who's stats are broken via name changes
- [x] 7 - make team deciding algorithm (T teams)
    - [x] 7.1 - strongest T players are 'captains', index these in order chosen
    - [x] 7.2 - from last to to first, sort next T strongest players into teams
    - [x] 7.3 - from first to last, sort next T strongest players into teams
    - [x] 7.4 - repeat from 7.2 until finished
- [x] 8 - check if teams make sense, printing team list and final score


- How to use:
    - bingo-teams make-players <csv with players usernames>
        - this would just get the players data from temple and export as csv
    - bingo-teams weight-players <csv with players and scores in csv format>
        - this would be used after manual changing of scores and fixing of broken players, outputs same csv but with a weighted score
    - bingo-teams make-teams <csv with players and full scores> <amount of teams to divide into>
        - this would be used once all scores are done, out puts a txt document with teams, members, final summed score, number of players in team


To-do
- ensure this process is clear
- change location of input/outputs, not nicely made atm
- make a config readable file to list what bosses/stats are important 
    - this config should also include weightings for these stats so the calculation can be manageable there
- make a different weighting for ironmen, so we take ironmen ehp/ehb and weight differently
- make printing functions generic
- make printing functions use serde for customizable outputs
- abstract out cli usage from the main
- make a constants file
- think about how to scale scores better
- skew skill xp data to fit xp between level for scoring -> https://oldschool.runescape.wiki/w/Experience
- 
