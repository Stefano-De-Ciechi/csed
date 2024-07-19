# CSED - Colored Sobel Edge Detection
Porting of the colored Sobel Edge Detection Algorithm found in [Milchreis imageprocessing](https://github.com/Milchreis/processing-imageprocessing) library made for the Processing environment. Credits go to him.

The results differ a little bit (contourns are a little "slimmer" in my version)

The algorithm is embedded in a simple cli program that takes a path to an image and produces the result.

# Installation:

git clone [url of this repository]
cd csed
cargo build --release
cargo run --release -- <path to image> [-o <result output>]

Be sure to compile in release mode, or else the program may run much slower (tested on a 1000x1500 .jpg image, debug took around 6 seconds, release about 0.25 seconds)

Currently tested only on Linux system; but if I used std::path::PathBuf correctly it should work cross-platform with no problems.
