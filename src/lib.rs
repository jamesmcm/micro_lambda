pub fn lambda(handler: fn(&str) -> std::result::Result<String, String>) {
    // Initialise one-time resources here
    // If initialisation error, POST to /runtime/init/error
    // Get new invocation events and pass to handler

    let aws_lambda_runtime_api = std::env::var("AWS_LAMBDA_RUNTIME_API").unwrap();
    loop {
        let invocation = ureq::get(&format!(
            "http://{}/2018-06-01/runtime/invocation/next",
            aws_lambda_runtime_api
        ))
        .call();

        let request_id = invocation
            .header("Lambda-Runtime-Aws-Request-Id")
            .unwrap()
            .to_string();

        let response = handler(invocation.into_string().unwrap().as_str());

        match response {
            Ok(res) => {
                let _resp = ureq::post(&format!(
                    "http://{}/2018-06-01/runtime/invocation/{}/response",
                    aws_lambda_runtime_api, request_id
                ))
                .send_string(&res);
            }

            Err(err) => {
                // Handle error
                // If invocation error, POST to /runtime/invocation/AwsRequestId/error

                let _resp = ureq::post(&format!(
                    "http://{}/2018-06-01/runtime/invocation/{}/error",
                    aws_lambda_runtime_api, request_id
                ))
                .send_string(&err.to_string());
            }
        }
    }
}
