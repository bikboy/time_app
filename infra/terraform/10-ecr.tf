resource "aws_ecr_repository" "timeapp" {
  name                 = "timeapp"
  image_tag_mutability = "MUTABLE"

  image_scanning_configuration {
    scan_on_push = true
  }
}