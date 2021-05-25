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



1
- just read in names
- output csv with scores, set manual scores to 0
- this is so i can set a flag to read in names or read in names+data csv, so no need to keep calling temple


