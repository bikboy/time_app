# Simple time application
## Build
`cargo build --release`
Application example of power and proficiency of RUST
## Routes:
* root (/) returns local time in New-York, Berlin, Tokyo in HTML format
* healthcheck (/health) return status code 200, in JSON format
## Dockerization
Application dockerized with new docker option "docker init"(more info in README.Docker.md). Total image footprint = 5MB
## Infrastructure
Infrastructure provisioned with Terraform (folder infra/terraform) :
* VPC
* Subnets
* SGs
* Routing tables
* EKS cluster
* ECR
## Deployment
Appliction deployed to EKS with helm chart(infra/helm/timeapp)
## Access
currently application available with temporary link: http://a186f1bc91e3a4f238e9cb8d6683f320-165755754.us-east-1.elb.amazonaws.com:8080/
