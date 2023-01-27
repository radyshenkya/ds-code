To run go trough that steps:
1. Pull docker image called "python"
2. Create docker network with this command:
```sh
docker network create --internal --subnet 10.1.1.0/24 ds-user-code-network
```