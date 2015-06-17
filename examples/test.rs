
extern crate plutchik;
extern crate rand;

fn main() {
    use plutchik::{Emotion, EMOTIONS, Wheel};

    println!("Robert Plutchik's wheel of emotions, typified!");

    println!("\nConversions");
    let start_emotion = Emotion::Ecstacy;
    let wheel = *start_emotion;
    println!("{:?} to {:?}", &start_emotion, &wheel);
    let vec: [f32; 2] = wheel.into();
    println!("{:?} to {:?}", &wheel, &vec);
    let wheel: Wheel = vec.into();
    println!("{:?} back to {:?}", &vec, &wheel);
    let end_emotion = wheel.closest_emotion();
    println!("{:?} back to {:?}", &wheel, &end_emotion);
    assert!(start_emotion == end_emotion);

    println!("\nAll Emotions");
    for &emotion in EMOTIONS.iter() {
        let vec: [f32; 2] = (*emotion).into();
        println!("{:?}\n\t{:?}\n\t{:?}", emotion, *emotion, vec);
    }

    println!("\nAll Emotion Comparisons");
    for emotion in EMOTIONS.iter() {
        for other in EMOTIONS.iter() {
            println!("Difference between {:?} and {:?}: {:?}%",
                     emotion, other, emotion.difference(other) * 100.0);
        }
    }

    println!("\nClosest Emotions");
    let some_wheel = Wheel { radians: 1.93, weight: 0.4 };
    println!("The closest Emotion to {:?} is {:?}.", some_wheel, some_wheel.closest_emotion());
    println!("Their difference is {:?}%.", some_wheel.closest_emotion().difference(&some_wheel) * 100.0);
    println!("The closest 5 emotions are {:?}", some_wheel.closest_emotions(5));
    println!("The opposite is {:?} which is closest to {:?}.",
             some_wheel.opposite(), some_wheel.opposite().closest_emotion());

    println!("\nMean (Average) Emotion");
    let emotions = [Emotion::Ecstacy, Emotion::Serenity, Emotion::Admiration, Emotion::Acceptance];
    let mean = Wheel::mean(&emotions);
    assert!(mean.closest_emotion() == Emotion::Love);
    println!("The mean emotion of Ecstasy, Serenity, Admiration and Acceptance is {:?}! AKA {:?}.",
             mean.closest_emotion(), mean);

    let emotions: Vec<Emotion> = (0..4).map(|_| rand::random()).collect();
    let mean = Wheel::mean(&emotions);
    println!("The mean emotion of {:?}, {:?}, {:?} and {:?} is {:?}. AKA {:?}.",
             emotions[0], emotions[1], emotions[2], emotions[3], mean.closest_emotion(), mean);
}

