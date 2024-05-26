variable "github_token" {
  description = "Github token."
  type        = string
  sensitive   = true
}

variable "github_owner" {
  description = "Github Account Owner(Username/Organization)"
  type        = string
}
