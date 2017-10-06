# Creating .dca files (Windows)

DCA specification can be found at https://github.com/bwmarrin/dca

Code and instructions from https://github.com/jonas747/dca/tree/master/cmd/dca

## Installation

Install Go and ffmpeg 

Add Go and ffmpeg `/bin` directories to your system path.

`go get github.com/jonas747/dca/cmd/dca`

dca should now be built in `%USERPROFILE%/go/bin`

## Example usage
`dca.exe in.wav > out.dca`

Input file can also be piped to dca.exe.