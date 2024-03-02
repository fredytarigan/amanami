# Amanami

A simple CLI application to check available updates for a certain things.

Help day to day DevOps job to easily get available updates related to infrastructure.

## How it works

This application will scrape configured target to check if there is any available updates.

It will compare the current resources will it's upstream latest version, and show if update is availabe.

Amanami consist of some subcommands with it's own function.

**To show global help**

```bash
$ amanami --help

Usage: amanami [OPTIONS] <COMMAND>

Commands:
  eks   AWS EKS update check
  help  Print this message or the help of the given subcommand(s)

Options:
  -f, --config-file <CONFIG_FILE>  Config file
                                   Application will look for "config/config.yaml" if this option isn't specified
  -h, --help                       Print help
```

**To show specific subcommand help**

```bash
$ amanami eks --help

AWS EKS update check

Usage: amanami eks

Options:
  -h, --help  Print help
```

## Features

### AWS EKS

- EKS Cluster Update List
- Node Groups Update List
- Addons Update List

## Configuration

### EKS

For configuring the application, please refer to [CONFIG](config/config.yaml.example) file.

#### Single Cluster

Single cluster configuration.

```
---
app:
  name: Amanami
  version: 0.0.1
aws:
  - account_id: 111111111111
    eks:
      - cluster_name: eks-cluster-a
        region: us-east-1
```

#### Multiple Cluster

Multiple cluster inside a single account configuration.

```
---
app:
  name: Amanami
  version: 0.0.1
aws:
  - account_id: 111111111111
    eks:
      - cluster_name: eks-cluster-a
        region: us-east-1
      - cluster_name: eks-cluster-b
        region: us-west-1
```

#### Multiple Cluster - Multiple AWS Account

Multiple cluster in multiple aws account configuration.

```
---
app:
  name: Amanami
  version: 0.0.1
aws:
  - account_id: 111111111111
    eks:
      - cluster_name: eks-cluster-a
        region: us-east-1
  - account_id: 222222222222
    role_arn: arn:aws:iam::222222222222:role/amanami-role
    eks:
      - cluster_name: eks-cluster-b
        region: us-east-1
      - cluster_name: eks-cluster-c
        region: us-west-1
  - account_id: 333333333333
    role_arn: arn:aws:iam::333333333333:role/amanami-role
    eks:
      - cluster_name: eks-cluster-d
        region: us-east-2
      - cluster_name: eks-cluster-e
        region: eu-west-1
```

## License

Copyright 2024 the Amanami Authors, All rights reserved.

Licensed under the [GNU GPL v3](https://www.gnu.org/licenses/gpl-3.0.html)

See the [LICENSE](LICENSE) at this repository.
