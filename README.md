# Satwatch 
A 3D Solar System Visualisation written in Rust

## About Satwatch 
This project is created with the following goals in mind 
  * Get a better understanding about Rust as a language. Aka: Learn Rust better. 
  * Learn more about the way Positioning is handled in Professional space applications. 
  * Create a tool to look at current and past missions, just for the fun of it. 
  * Talking about fun: FUN! 

## Installation 
At some point there will be prebuilt binaries, but as of know, need to build this yourself. 
  * Clone the repository and its submodules. `git clone git@github.com:mkalte666/satwatch --recursive`
  * optional: build it with cargo: `cargo build --release`
  * Run it with cargo `cargo run --release`

You will need a c and c++ compiler installed for some dependencies to build correctly. 

## TODO
The todo, until this gets close to something resembling a release,is rather big.
This is not exhaustive, but for a "1.0" these are the features to go for 

####Earth satellites 
 * [x] SGP4 based TLE propagation 
 * [ ] UI and drawing improvements for the ui related to this 

####Directly SPICE related
 * [x] NASA-SPICE in the build
 * [ ] SPICE-Based Time 
 * [ ] SPICE-Based (using PDS Data) Planet positions
 * [ ] Arbitrary spice objects 
 * [ ] An integrated downloader for NASA PDS Data
 * [ ] Correct rotations of all planets (and maybe the sun, if its texture is good enough)
 
####Other functionality 
 * [ ] Observation(aka passes) finder from 
 * [ ] Lat/Long/Altitude coordinates 
 * [ ] Planets are not spheres
 * [ ] Textures for all panets and the sun 
 * [ ] Switch between the normal equator-based camera and one that has its plane on the ecliptic
 * [ ] More consistent UI. Probably best tackled somewhere in between "nearly everything is prototyped" and "too late to fix the current state"
 * [ ] Some sort of welcome-screen that takes care of the initial requred downloads (Planetary ephemeris and leap second kernel)

####More basic stuff 
 * [ ] UV-Mapped Textures for objects 
 * [ ] Some form of view-frustrum culling, maybe, if needed on weaker machines. I doubt it, needs testing. 
 * [ ] Orbit camera instead of the FPS one currently implemented. Would then probably play better with the euler singularities as well. Were not a spaceship :(

####Not directly code related 
 * [ ] Some cool screenshots for this readme 
 * [ ] CI builds 
 * [ ] Write a blog post or talk when this is finished, explaning the whole thing. 
 * [ ] Same as above, but for people touching the code: Documentation.  