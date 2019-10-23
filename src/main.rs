extern crate config;
extern crate reqwest;
extern crate serde;
extern crate serde_derive;
extern crate serde_json;
extern crate serde_xml_rs;

use std::process;

use serde::{Serialize, Deserialize};

#[derive(Debug)]
pub enum ResponseType {
    Version         { response: Option<VersionResponse> },
    Broker          { response: Option<BrokerResponse> },
    HeapMemoryUsage { response: Option<HeapMemoryUsageResponse> },
    Queues          { response: Option<QueuesResponse> },
    Topics          { response: Option<TopicsResponse> },
    Subscribers     { response: Option<SubscribersResponse> },
}

fn send_request(t: ResponseType, req: String, settings: &config::Config) -> Result<ResponseType, String> {
    // Auth
    let user = settings.get::<String>("username").unwrap();
    let pass = settings.get::<String>("password").unwrap();

    let client = reqwest::Client::new();
    let mut resp = client.get(req.as_str()).basic_auth(&user, Some(&pass)).send().unwrap();

    let buf = resp.text().unwrap();
    
    match t {
        ResponseType::Broker {response: _} => {
            let res: BrokerResponse = match serde_json::from_str(&buf) {
                Ok(v) => v,
                Err(e) => {
                    println!("failed deserializing json response: {}", e);
                    process::exit(1);
                },
            };
            Ok(ResponseType::Broker { response: Some(res) })
        },
        ResponseType::Version {response: _} => {
            let res: VersionResponse = match serde_json::from_str(&buf) {
                Ok(v) => v,
                Err(e) => {
                    println!("failed deserializing json response: {}", e);
                    process::exit(1);
                },
            };
            Ok(ResponseType::Version { response: Some(res) })
        },
        ResponseType::HeapMemoryUsage {response: _} => {
            let res: HeapMemoryUsageResponse = match serde_json::from_str(&buf) {
                Ok(v) => v,
                Err(e) => {
                    println!("failed deserializing json response: {}", e);
                    process::exit(1);
                },
            };
            Ok(ResponseType::HeapMemoryUsage { response: Some(res) })
        },
        ResponseType::Queues{response: _} => {
            // TODO xml deserialization.
            let res: QueuesResponse = match serde_xml_rs::from_str(&buf) {
                Ok(v) => v,
                Err(e) => {
                    println!("failed deserializing json response: {}", e);
                    process::exit(1);
                },
            };
            Ok(ResponseType::Queues { response: Some(res) })
        },
        ResponseType::Topics{response: _} => {
            // TODO xml deserialization.
            let res: TopicsResponse = match serde_json::from_str(&buf) {
                Ok(v) => v,
                Err(e) => {
                    println!("failed deserializing json response: {}", e);
                    process::exit(1);
                },
            };
            Ok(ResponseType::Topics { response: Some(res) })
        },
        ResponseType::Subscribers{response: _} => {
            // TODO xml deserialization.
            let res: SubscribersResponse = match serde_json::from_str(&buf) {
                Ok(v) => v,
                Err(e) => {
                    println!("failed deserializing json response: {}", e);
                    process::exit(1);
                },
            };
            Ok(ResponseType::Subscribers { response: Some(res) })
        },
    }
}

fn api_get_broker(settings: &config::Config) {
    // Broker url
    // Eg. http://localhost:8161/api/jolokia/read/org.apache.activemq:type=Broker,brokerName=localhost
    let url = "api/jolokia/read/org.apache.activemq:type=Broker,brokerName=";
    let req = format!("http://{}:{}/{}{}", 
                settings.get::<String>("hostname").unwrap(),
                settings.get::<String>("brokerport").unwrap(),
                url,
                settings.get::<String>("brokername").unwrap(),
                );

    let result = match send_request(ResponseType::Broker{response: None}, req, &settings) {
        Ok(v)   => v,
        Err(e)  => {
            println!("error: request failed: {}", e);
            process::exit(1);
        },
    };

    // Err(String::from("err happened"));

    println!("Result broker:");
    println!("{:#?}", result);
}

fn api_get_version(settings: &config::Config) {
    // API version url
    // Eg. http://localhost:8161/api/jolokia
    let req = format!("http://{}:{}/{}/{}", 
        settings.get::<String>("hostname").unwrap(),
        settings.get::<String>("brokerport").unwrap(),
        "api",
        "jolokia",
    );

    let result = match send_request(ResponseType::Version{response: None}, req, &settings) {
        Ok(v)   => v,
        Err(e)  => {
            println!("error: request failed: {}", e);
            process::exit(1);
        },
    };

    println!("Result version:");
    println!("{:#?}", result);
}

fn api_get_heap_memory_usage(settings: &config::Config) {
    // Heap Memory Usage
    // Eg. http://localhost:8161/api/jolokia/read/java.lang:type=Memory/HeapMemoryUsage
    let req = format!("http://{}:{}/{}/{}/{}/{}/{}", 
                settings.get::<String>("hostname").unwrap(),
                settings.get::<String>("brokerport").unwrap(),
                "api",
                "jolokia",
                "read",
                "java.lang:type=Memory",
                "HeapMemoryUsage",
                );

    let result = match send_request(ResponseType::HeapMemoryUsage{response: None}, req, &settings) {
        Ok(v)   => v,
        Err(e)  => {
            println!("error: request failed: {}", e);
            process::exit(1);
        },
    };

    println!("Result heap:");
    println!("{:#?}", result);
}

// TODO implement the following endpoint requests
/*
http://localhost:8161/admin/xml/topics.jsp
http://localhost:8161/admin/xml/subscribers.jsp
http://localhost:8161/admin/queueBrowse/RTestQ?view=rss&feedType=rss_2.0
http://localhost:8161/admin/queueBrowse/RTestQ?view=rss&feedType=atom_1.0
http://localhost:8161/admin/queueBrowse/RTestQ
*/

fn api_get_queues(settings: &config::Config) {
    // Queues
    // Eg. http://localhost:8161/admin/xml/queues.jsp
    let req = format!("http://{}:{}/{}/{}/{}", 
                settings.get::<String>("hostname").unwrap(),
                settings.get::<String>("brokerport").unwrap(),
                "admin",
                "xml",
                "queues.jsp",
                );

    let result = match send_request(ResponseType::Queues{response: None}, req, &settings) {
        Ok(v)   => v,
        Err(e)  => {
            println!("error: request failed: {}", e);
            process::exit(1);
        },
    };

    println!("Result queues:");
    println!("{:#?}", result);
}

fn main() {
    let mut settings = config::Config::default();
    settings.merge(config::File::with_name("Settings")).unwrap();

    api_get_broker(&settings);

    api_get_version(&settings);

    api_get_heap_memory_usage(&settings);

    api_get_queues(&settings);

}

// Queues
#[derive(Serialize, Deserialize, Debug)]
pub struct QueuesResponse {
    // #[serde(rename = "queue", default)]
    pub queue: Vec<QueuesQueue>,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct QueuesQueue {
    pub name: String,
    pub feed: QueuesQueueFeed,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct QueuesQueueFeed {
    pub atom: String,
    pub rss: String,
}

// Topics
#[derive(Serialize, Deserialize, Debug)]
pub struct TopicsResponse {
    //
}

// Subscribers
#[derive(Serialize, Deserialize, Debug)]
pub struct SubscribersResponse {
    //
}

// API Version
#[derive(Serialize, Deserialize, Debug)]
pub struct VersionResponse {
    request: VersionRequest,
    status: i32, 
    timestamp: u64, 
    value: VersionValue, 
}
#[derive(Serialize, Deserialize, Debug)]
struct VersionRequest {
    #[serde(rename="type")]
    request_type: String,
}
#[derive(Serialize, Deserialize, Debug)]
struct VersionValue {
    agent : String,
    protocol: String,
    config: VersionValueConfig,
    info: VersionValueInfo,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all="camelCase")]
struct VersionValueConfig {
    agent_id : String,
    agent_type: String,
    allow_error_details: String,
}
#[derive(Serialize, Deserialize, Debug)]
struct VersionValueInfo {
    product: String,
    vendor: String,
    version: String,
}

// Heap Memory Usage
#[derive(Serialize, Deserialize, Debug)]
pub struct HeapMemoryUsageResponse {
    request: HeapMemoryUsageRequest,
    status: i32, 
    timestamp: u64, 
    value: HeapMemoryUsageValue, 
}
#[derive(Serialize, Deserialize, Debug)]
struct HeapMemoryUsageRequest {
    mbean: String, 
    attribute: String,
    #[serde(rename="type")]
    request_type: String,
}
#[derive(Serialize, Deserialize, Debug)]
struct HeapMemoryUsageValue {
    init: usize, 
    committed: usize,
    max: usize,
    used: usize,
}

// Broker
#[derive(Serialize, Deserialize, Debug)]
pub struct BrokerResponse {
    request: BrokerRequest,
    value: BrokerValue, 
    timestamp: u64, 
    status: i32, 
}
#[derive(Serialize, Deserialize, Debug)]
struct BrokerRequest {
    mbean: String, 
    #[serde(rename="type")]
    request_type: String,
}
#[derive(Serialize, Deserialize, Debug)]
struct Object {
    #[serde(rename="objectName")]
    object_name: String
}
#[derive(Serialize, Deserialize, Debug)]
struct TransportConnector {
    amqp: String,
    mqtt: String,
    openwire: String,
    stomp: String,
    ws: String,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all="PascalCase")]
struct BrokerValue {
    average_message_size: u64,
    broker_id: String,
    broker_name: String,
    broker_version: String,
    current_connections_count: u16,
    data_directory: String,
    durable_topic_subscribers: Vec<Object>,
    dynamic_destination_producers: Vec<Object>,
    inactive_durable_topic_subscribers: Vec<Object>,
    jms_job_scheduler: Option<String>, // ? null,
    job_scheduler_store_limit: u64,
    job_scheduler_store_percent_usage: u8,
    max_message_size: u64,
    memory_limit: u64,
    memory_percent_usage: u8,
    min_message_size: u16,
    persistent: bool,
    queue_producers: Vec<Object>,
    queue_subscribers: Vec<Object>,
    queues: Vec<Object>,
    slave: bool,
    statistics_enabled: bool,
    store_limit: u64,
    store_percent_usage: u8,
    temp_limit: u64,
    temp_percent_usage: u8,
    temporary_queue_producers: Vec<Object>,
    temporary_queue_subscribers: Vec<Object>,
    temporary_queues: Vec<Object>,
    temporary_topic_producers: Vec<Object>,
    temporary_topic_subscribers: Vec<Object>,
    temporary_topics: Vec<Object>,
    topic_producers: Vec<Object>,
    topic_subscribers: Vec<Object>,
    topics: Vec<Object>,
    total_connections_count: u64,
    total_consumer_count: u16,
    total_dequeue_count: u64,
    total_enqueue_count: u64,
    total_message_count: u64,
    total_producer_count: u64,
    transport_connectors: TransportConnector,
    uptime: String,
    uptime_millis: u64,

    #[serde(rename="VMURL")]
    vm_url: String,
}
