use toml::Value;

pub fn read_config() {
    let content = r#"
        ip = '127.0.0.1'

        [nats]
        name = 'potato'

        [nats.child]
        name = 'potato child 1'

        [nats.child2]
        name = 'potato child 2'

        [nats.child.child]
        name = 'potato child child 1'
    "#;

    let value = content.parse::<Value>().unwrap();

    for a in value.as_table() {
        for k in a.keys().collect::<Vec<&String>>() {
            let item = a.get(k).unwrap();

            if let Some(_table_item) = item.as_table() {
                println!("{} its a nested table!", k);
            } else {
                println!("{} is a leaf!", k);
            }
        }
    }
}
