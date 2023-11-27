# Filler
- Project description: https://github.com/01-edu/public/tree/master/subjects/filler
- Download docker image from https://assets.01-edu.org/filler/filler.zip
- To build the image `docker build -t filler .`
- To run the container `docker run -v "$(pwd)/solution":/filler/solution -it filler`. This instruction will open a terminal in the container, the directory `solution` will be mounted in the container as well.
- Example of a command in the container `./linux_game_engine -f maps/map01 -p1 linux_robots/bender -p2 linux_robots/terminator`
- Your solution should be inside the `solution` directory so it will be mounted and compiled inside the container and it will be able to be run in the game engine.

## Notes

- `Terminator` is a very strong robot so it's optional to beat him.
- For M1 Macs use `m1_robots` and `m1_game_engine`.
- cd solution/filler, cargo build, then:
- `../../m1_game_engine -f ../../maps/map02 -p2 ../../m1_robots/terminator -p1 target/debug/filler`
- change terminator to any bot you want like bender, h2_d2 or wall_e :D
- If you get permission errors do `find . -type f -exec chmod +x {} \;` in root

