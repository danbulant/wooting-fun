# Bad Apple on Wooting

Tested on Wooting two HE ARM.

## Requirements

Install `wooting-rgb-sdk`. On arch, it's available in the AUR.  
Enable udev rules for wooting if you haven't done so (you can install wootility-lekker from AUR which contains the rules). You will need to run the app as root if you don't.  
For compilation, just run `cargo build` (called automatically when running with `cargo run`). If you don't have rust installed, you can install it with [`rustup`](https://rustup.rs/).

## Playback

App reads from `data/resized` and displays each image (sorted by ASCII) for 1/30th of a second on the keyboard directly.
This requires the image to be resized correctly as the app doesn't do any resizing.

To set up your video, create the `data` folder, download your video there, extract frames with ffmpeg and resize them with imagemagick:

The following is a fish script I used:

```bash
mkdir data && cd data
yt-dlp <video URL here> # check for legality in your state :)
ffmpeg -i <video file> -r 30 data/%04d.png
for i in (find out/ | string sub -s 5);
    magick out/$i -resize 7x6! resized/$i;
end
```

Bad Apple for example can roughly be translated to a 7x6 grid (6 is height, which is the same as the number of rows on the keyboard). the `!` force resize to the specified size.

Then start the command - if you have the repository clonned locally, use `cargo run` (rust required, see rustup for installation of rust), or run the binary.
