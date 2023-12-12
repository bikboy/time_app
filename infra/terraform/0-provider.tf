provider "aws" {
  region = "us-east-1"
}

terraform {
  required_providers {
    aws = {
      source  = "hashicorp/aws"
      version = "~> 3.0"
    }
  }
}
terraform {
  backend "s3" {
    bucket = "timeapptfstate"
    key    = "state"
    region = "us-east-1"
  }
}