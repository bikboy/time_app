# Simple time application
Application example of power and proficiency of RUST
## Build
`cargo build --release`
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
`helm install timeapp infra/helm/timeapp -f infra/helm/timeapp/values.yaml`
## Access
currently application not available (terraform destroy)