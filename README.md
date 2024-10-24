# Animancer

## Credits
Programming/Design - Joel Wakefield
Art - Aelita "Vessk" Mullet
Music - Marce R

## Task List
- [x] user can select a single unit
- [x] user can select multiple units
- [x] user can move selected unit(s)
- [x] selected units move as a group (they have a buffer to distance themselves by)
- [x] use can hold the right mouse button and "aim" the direction selected units face
- [x] user can change the spacing of units in a formation by the magnitude of the aim direction
- [x] user can change a groups formation 
  - [x] ringed
  - [x] line
  - [x] box
  - [x] NO ~staggered~
- [x] selection should store selected entities for better control of formations
- [x] user can spawn units from a spawner
- [x] units move to building when placing
- [x] user can queue multiple units from producer
- [x] user can set post-spawn location
- [x] user can see post-spawn location
- [x] buildings cost resources (faith)
- [x] buildings are constructed before existing
- [x] user can see a silhouette of the building to be constructed
- [x] user can assign selected units to "construct" buildings
- [x] user can assign new units to construction site
- [x] not needed ~user can remove units from construction site~
- [x] user should be able to place one building with left click, or hold shift + click to plant another
- [x] user can direct units to generate "faith" (currency)
- [x] navmesh [bevy_pathmesh](https://docs.rs/bevy_pathmesh/latest/bevy_pathmesh/)
  - [x] effect unit movement
  - [x] be effected by buildings
- [x] user should be able to see the building silhouette of a building before placing it
- [x] user should not be able to layer buildings
- [ ] units should be an `Obstacle` on the nevmesh
  - [ ] performance is stable
  - [ ] unit movement is still clean and "flowy"
- [x] user should be able to move about a map
- [ ] game map
  - [ ] actual map, with natural obstacles
  - [ ] minimap
- [ ] user should be able to see unit generation in the ui
- [ ] user should be able to "snap" buildings together if close enough
- [ ] user should be able to rotate buildings when placing them
- [ ] user should be able to win the game
  - [ ] obliteration (enemy cannot do the following)
    - [ ] cannot make new units
    - [ ] no more units
  - [ ] capture the flag
    - [ ] hold some number of capture points
  - [ ] king of the hill
    - [ ] control region(s) for a constistant amount of time
  - [ ] regicide
    - [ ] destroy the enemy hero