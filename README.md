micro_lambda
============

This crate is a small tutorial example of creating a [custom AWS Lambda
runtime](https://docs.aws.amazon.com/lambda/latest/dg/runtimes-custom.html#runtimes-custom-build).

The process is very simple, the AWS Lambda Runtime API currently
contains [only four endpoints](https://docs.aws.amazon.com/lambda/latest/dg/runtimes-api.html):

* Initalization error - for if the initialization steps fail (i.e.
  doing one-time initializations for global resources, etc.) prior to
  calling the handler function.
* Next invocation - an endpoint from which to GET the invocation event and
  some metadata (AWS Request ID).
* Invocation response - an endpoint to POST the successful response of
  the handler function.
* Invocation error - an endpoint to POST the error message of the
  handler function, if it fails.

AWS Lambda also provide a [simple tutorial](https://docs.aws.amazon.com/lambda/latest/dg/runtimes-walkthrough.html)
using bash.

An extension of this would be to add support for the [AWS Lambda
Extensions API](https://docs.aws.amazon.com/lambda/latest/dg/runtimes-extensions-api.html),
but in the real world you should just use the [lambda-runtime](https://github.com/awslabs/aws-lambda-rust-runtime) crate
 (which also provides support for async handler functions!).
