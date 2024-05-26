provider "github" {
  token = var.github_token
  owner = var.github_owner
}

resource "github_repository" "wspr_cdk" {
  name        = "wspr_cdk"
  description = "This crate provides an abstraction that allows you to do analysis on wspr's real time spot data."
  visibility  = "public"
}
