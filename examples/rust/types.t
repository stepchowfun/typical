struct SendEmailRequest {
    to: String = 0
    subject: String = 1
    body: String = 2
}

choice SendEmailResponse {
    success = 0
    error: String = 1
}
