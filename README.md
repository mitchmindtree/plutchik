
# plutchik [![Build Status](https://travis-ci.org/mitchmindtree/plutchik.svg?branch=master)](https://travis-ci.org/mitchmindtree/plutchik)

A small type representation of [Robert Plutchik's](https://en.wikipedia.org/wiki/Robert_Plutchik) ["Wheel of Emotions"](https://en.wikipedia.org/wiki/Contrasting_and_categorization_of_emotions#/media/File:Plutchik-wheel.svg).

- Use labeled `Emotion`s (i.e. Emotion::Ecstacy, Emotion::Terror, etc).
- Design custom emotions using the `Wheel { radians: f32, weight: f32 }` representation.
- Find the difference between two emotions (the magnitude of the vector that separates them on the Wheel).
- Find the mean emotion of multiple given emotions i.e.
```Rust
assert!(Wheel::mean(&[Serenity, Acceptance, Joy, Trust]).closest_emotion() == Love);
```

![Image of the Wheel of Emotions](https://upload.wikimedia.org/wikipedia/commons/thumb/c/ce/Plutchik-wheel.svg/715px-Plutchik-wheel.svg.png)

For a demo, see the [example](https://github.com/mitchmindtree/plutchik/blob/master/examples/test.rs).

You can add it to your project by adding this to your Cargo.toml:

```toml
[dependencies]
plutchik = "*"
```


