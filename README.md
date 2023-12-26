# brickpack-2023
## [ARCHIVED] - Recommend using (https://loco.rs/)

## Devcontainer issues related to docker 
To solve docker issues related to "Temporary failure resolving"
We need to specify a DNS server for docker containers.

Create a /etc/docker/daemon.json file with this content:
```json
{
  "dns": ["8.8.8.8", "8.8.4.4"]
}
```
