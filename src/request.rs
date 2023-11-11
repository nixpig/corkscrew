use crate::config::Config;

pub fn exec(config: &Config) {
    config.hosts.iter().for_each(|x| {
        x.requests
            .iter()
            .filter(|x| {
                if config.request_names.len() != 0 {
                    config.request_names.contains(&x.name)
                } else {
                    true
                }
            })
            .for_each(|x| {
                println!("send request: {:?}", x);
            });
    })
}
