use crate::config::Config;

pub fn exec(config: &Config) {
    config.hosts.iter().for_each(|x| {
        x.requests
            .iter()
            .filter(|x| config.request_names.contains(&x.name))
            .for_each(|x| {
                println!("send request: {:?}", x);
            });
    })
}
