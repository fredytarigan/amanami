mod config;

use crate::config::AwsConfig;
use config::Config;
use std::thread;

#[derive(Debug, Clone)]
pub struct Aws {
    aws_account: Vec<AwsAccount>,
}

#[derive(Debug, Clone)]
pub struct AwsAccount {
    account_id: String,
    role_arn: Option<String>,
    eks: Option<Vec<EksConfig>>,
}

#[derive(Debug, Clone)]
pub struct EksConfig {
    cluster_name: String,
    region: String,
}

impl Aws {
    pub fn new(config: Vec<AwsConfig>) -> Self {
        let mut aws: Vec<AwsAccount> = Vec::new();

        for account in config {
            let mut eks: Vec<EksConfig> = Vec::new();

            match account.eks {
                Some(d) => {
                    for item in d {
                        let eks_config = EksConfig {
                            cluster_name: item.cluster_name,
                            region: item.region,
                        };

                        eks.push(eks_config);
                    }
                }
                None => (),
            }

            let eks_data = match eks.len() {
                0 => None,
                _ => Some(eks),
            };

            let aws_account = AwsAccount {
                account_id: account.account_id,
                role_arn: account.role_arn,
                eks: eks_data,
            };

            aws.push(aws_account);
        }

        Self { aws_account: aws }
    }

    pub fn get_eks_updates(&self) -> Result<(), std::io::Error> {
        // prepare thread
        let mut v = Vec::<std::thread::JoinHandle<()>>::new();

        // loop over all aws account
        for account in self.aws_account.clone() {
            let thread = thread::spawn(move || {
                let config = Config::new(String::from("ap-southeast-1"), account.role_arn);
                let resp = config.generate_config();

                println!("{:?}", resp);
                println!("");
                println!("");
            });
            v.push(thread);
        }

        for item in v.into_iter() {
            item.join().unwrap();
        }

        unimplemented!();
    }
}
