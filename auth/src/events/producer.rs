use rdkafka::config::ClientConfig;
use rdkafka::producer::{BaseProducer, BaseRecord, Producer};
use std::time::Duration;
use std::thread;

// fn get_vars() {
//     // let keys = vec::["topic", "username", "password"];
//     let key_val = HashMap::new();

// }


pub fn call_producer() {

    let  producer: BaseProducer = ClientConfig::new()
        .set("bootstrap.servers", "localhost:9092")
        .set("security.protocol", "SASL_SSL")
        .set("sasl.mechanims", "PLAIN")
        .set("sasl.username", "<username>")
        .set("sasl.password", "<password>")
        .create()
        .expect("Producer Error: Invalid Producer Config");


    // Poll at regular intervals to process all the asynchronous delivery events.
    for i in 0..100 {
    // producer.flush(Duration::from_secs(1))
    println!("sending message");
    producer.send(
        BaseRecord::to("rust")
            .key(&format!("key-{}", i))
            .payload(&format!("value-{}", i)),
    ).expect("failed to send message");
    thread::sleep(Duration::from_secs(3));
}

// And/or flush the producer before dropping it.
producer.flush(Duration::from_secs(1));



}