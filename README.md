# Github commit history art generator
This program creates a repository that contains a bunch of commits which tricks Github into showing the specified commit history art.

## How to use?
1. Clone this repository.
2. Create a 52x7 image with the desired art.
3. Open `src/config.rs` and edit the art image path and year.
    <br>
    File `src/config.rs`:
    ```rust
    pub const YEAR: i32 = 2023;

    /// Image dimensions should be 52x7
    pub const IMAGE_PATH: &str = "image.png";

    pub const COMMITS_PER_WHITE_PIXEL: u32 = 3;
    ```
4. Run the program with `cargo run`.
<br> *This will generate a new repository with the name `repo` in the root directory of this project.*
5. Push the repository to Github.
6. Enjoy your art!