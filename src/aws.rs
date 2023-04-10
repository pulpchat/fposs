use aws_config::meta::region::RegionProviderChain;
use aws_sdk_dynamodb as dynamodb;
use dynamodb::types::{
    AttributeDefinition, KeySchemaElement, KeyType, ProvisionedThroughput, ScalarAttributeType,
};

async fn aws() -> Result<(), dynamodb::Error> {
    let region_provider = RegionProviderChain::default_provider().or_else("us-east-1");
    let config = aws_config::from_env().region(region_provider).load().await;
    let client = dynamodb::Client::new(&config);

    let a_name: String = "key".into();
    let table_name: String = "table".into();

    let ad = AttributeDefinition::builder()
        .attribute_name(&a_name)
        .attribute_type(ScalarAttributeType::S)
        .build();

    let ks = KeySchemaElement::builder()
        .attribute_name(&a_name)
        .key_type(KeyType::Hash)
        .build();

    let pt = ProvisionedThroughput::builder()
        .read_capacity_units(10)
        .write_capacity_units(5)
        .build();

    match client
        .create_table()
        .table_name(&table_name)
        .key_schema(ks)
        .attribute_definitions(ad)
        .provisioned_throughput(pt)
        .send()
        .await
    {
        Ok(_) => println!("Added table {} with key {}", a_name, table_name),
        Err(e) => {
            println!("Got an error creating table:");
            println!("{}", e);
            std::process::exit(1);
        }
    };

    Ok(())
}
