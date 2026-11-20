# Sound

## WAV import

**Midra Quest** has the ability to *import WAV data* (thanks to **hound** for WAV parsing ...),
but there are 3 restrictions:

1. The file should have *2 byte signed integer samples* (**i16**)
2. The file must have 1 or 2 channels (**Mono** or **Stereo**)
3. The file must fit to the RAM (as it is ***directly loaded to the RAM without streaming***)


