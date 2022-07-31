use rdkafka::client::ClientContext;
use rdkafka::producer::ProducerContext;
use rdkafka::producer::DeliveryResult;

struct ProducerCallbackLogger;

impl ProducerContext for ProducerCallbackLogger {
    type DeliveryOpaque = ();
    fn delivery(
        &self, 
        delivery_result: &DeliveryResult<'_>,
        _delivery_opaque: Self::DeliveryOpaque,
    ) {}
}


impl ClientContext for ProducerCallbackLogger {}

