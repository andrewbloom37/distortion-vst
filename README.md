# distortion-vst

A vst audio plugin for performing complex digital clipping.

## building

To build, [install rust/cargo](https://www.rust-lang.org/) and run `cargo build` in the project root.

If on macOS, a `build_vst` script is provided which will build the vst and output it in your `~` folder. To use this, move it to your `Library/Audio/VST/` folder. Run this script in from the project root folder, otherwise it will not work properly.

## parameters

### threshold

The portion of the positive part of the waveform which will be clipped

### lower_threshold

The portion of the negative part of the waveform which will be clipped

### fold

The amount of foldback which occurs... the waveform will be clipped based on the threshold values, and a scaled amount that was clipped will be subtracted back.

### gain

Use this to trim the audio if it is clipping your channel output
