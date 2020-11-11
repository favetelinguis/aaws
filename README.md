# aaws
A very thin wrapper for running profiles concurrently with AWS CLI 

### Prerequisites
Require AWS CLI is installed since the scripts just wraps the cli and provide concurrent execution for multiple profiles

### Installation
* Install the Rust toolchain
* Clone the repo
* cd into repo
* Run `cargo install --path .`

### Usage
`aaws 's3 ls' profile1 profile2 ...`

