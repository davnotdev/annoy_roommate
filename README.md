# Annoy Roommates

Here's a messy piece of code I hacked together in a couple hours to annoy my roommates.

If you are like me, and you self host on a server laptop under your desk, you can use this to: 

1. Play obnoxious TTS messages remotely
2. Play meme sound effects remotely
3. Play personal voice messages remotely (from mobile devices)

## Usage

On your self hosted box, 

1. Install `espeak` and `ffplay`
2. Ensure your audio server is running
3. Start the server!

```
cargo r -- localhost:3000 PASSWORD_HERE
```

4. Visit `localhost:3000` and enjoy.

