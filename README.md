# distortion-vst

A vst audio plugin for performing complex digital clipping.

## building

To build, [install rust/cargo](https://www.rust-lang.org/) and run `cargo build` in the project root.
Then, if compiling on macOS, run [this script](https://github.com/RustAudio/vst-rs/blob/master/osx_vst_bundler.sh)
and copy the vst to `Library/Audio/Plug-Ins/VST/`. Then it should register in your DAW of choice on next startup.

## parameters

### threshold

The portion of the positive part of the waveform which will be clipped

### lower_threshold

The portion of the negative part of the waveform which will be clipped

### scale

The amount of clipping which occurs... the waveform will be clipped, and a scaled amount that was clipped will be added back.

### gain

Use this to trim the audio if it is clipping your channel output
