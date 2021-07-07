fn main() {
    let mut cache = Vec::new();
    add_to_cache(&mut cache, Sample{time: 1, value: 1.0});
    println!("Current cache = {:?}", cache);

    add_to_cache(&mut cache, Sample{time: 2, value: 2.0});
    add_to_cache(&mut cache, Sample{time: 4, value: 4.0});
    println!("Current cache = {:?}", cache);

    add_to_cache(&mut cache, Sample{time: 0, value: 0.0});
    add_to_cache(&mut cache, Sample{time: 3, value: 3.0});
    println!("Current cache = {:?}", cache);

    for i in 0..255 {
        add_to_cache(&mut cache, Sample{time: i+5, value: 0.0});
    }
    println!("Current cache first value = {:?}", cache.get(0).unwrap());
    println!("Current cache last value = {:?}", cache.last().unwrap());
    assert_eq!(cache.len(), 255);
}

#[derive(Clone, Copy, Debug)]
struct Sample {
    time: u64,
    value: f64,
}

fn add_to_cache(cache: &mut Vec<Sample>, sample: Sample) {
    if cache.len() >= 255 {
        cache.drain(0..1);
    }
    if cache.len() == 0 || cache.get(cache.len()-1).unwrap().time < sample.time {
        cache.push(sample);
        return;
    }
    for idx in 0..cache.len() {
        if cache.get(idx).unwrap().time > sample.time {
            cache.insert(idx, sample);
            break;   
        } 
    }
}
