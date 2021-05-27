Todo
- [x] 1 - read in list of usernames
- [x] 2 - check they all can be called with temple osrs api first
- [x] 3 - decide good stats to check for
- [ ] 4 - create data structures for each player / teams
- [ ] 5 - decide weighting for player stats to score players
    - [ ] 5.1 - correctly calculate a average score
- [ ] 6 - check these scores all make sense, allow for manual override for outliers
    - [ ] 6.1 - find players who's stats are broken via name changes
- [ ] 7 - make team deciding algorithm (T teams)
    - [ ] 7.1 - strongest T players are 'captains', index these in order chosen
    - [ ] 7.2 - from last to to first, sort next T strongest players into teams
    - [ ] 7.3 - from first to last, sort next T strongest players into teams
    - [ ] 7.4 - repeat from 7.2 until finished
- [ ] 8 - check if teams make sense, printing team list and final score


- How to use:
    --make-players <csv with players usernames>
        - this would just get the players data from temple and export as csv
    --weight-players <csv with players and scores in csv format>
        - this would be used after manual changing of scores and fixing of broken players, outputs same csv but with a weighted score
    --make-teams <csv with players and full scores> --team-count <amount of teams to divide into>
        - this would be used once all scores are done, out puts a txt document with teams, members, final summed score, number of players in team

To-do
- have argument flags work
- abstract out functions
- make weighted score function
- make team distribution structures and functions
- make config file where we keep weights
- why the fuck is the release build so friggen big, 260mb for such small amount of code? does it include entire crates used?