# MediCompass

MediCompass is an advanced AI-powered application designed to provide personalized medication recommendations. It leverages the MeldRX API to connect to FHIR (Fast Healthcare Interoperability Resources) data, ensuring up-to-date and accurate healthcare information. The backend infrastructure of MediCompass is built using AWS Lambda (serverless) with Rust and DynamoDB for seamless performance and scalability.

## Table of Contents

- [Introduction](#introduction)
- [Features](#features)
- [Architecture](#architecture)
- [Prerequisites](#prerequisites)
- [Installation](#installation)
- [Usage](#usage)
- [API Reference](#api-reference)
- [Contributing](#contributing)
- [License](#license)

## Introduction

MediCompass aims to revolutionize how patients and healthcare providers access and utilize medication information. By integrating with the MeldRX API, the application can access comprehensive FHIR data to provide tailored medication recommendations, enhance treatment adherence, and improve overall healthcare outcomes.

## Features

- Personalized medication recommendations based on patient health data
- Integration with MeldRX API to access FHIR data
- Serverless architecture using AWS Lambda and Rust
- Scalable and low-latency data storage with DynamoDB
- Secure and compliant with healthcare regulations

## Architecture

The MediCompass application is built using a modern serverless architecture to ensure high availability, scalability, and cost-efficiency. The primary components of the architecture include:

- **MeldRX API**: Provides access to FHIR data.
- **AWS Lambda**: Executes application logic using Rust, enabling serverless computing.
- **DynamoDB**: Stores and retrieves patient data with low latency and high scalability.

```
            +--------------+
            | API Gateway  |
            +--------------+
                  |
         +--------+--------+
         |                 |
+--------v--------+ +--------v--------+
|  Lambda Func 1  | |  Lambda Func 2  |
+-----------------+ +-----------------+

```

## Prerequisites

Before you begin, ensure you have the following prerequisites:

- AWS account with necessary permissions
- Rust programming language installed
- AWS CLI installed and configured
- MeldRX API credentials
- DynamoDB table set up

## Installation

Follow these steps to set up and deploy the MediCompass application:

1. **Clone the repository:**

    ```sh
    git clone https://github.com/your-username/MediCompass.git
    cd MediCompass
    ```

2. **Install Rust dependencies:**

    ```sh
    cargo install
    ```

3. **Set up AWS Lambda:**

    Create a new AWS Lambda function using the AWS Management Console or AWS CLI. Ensure the function runtime is set to Rust.

4. **Deploy the Lambda function:**

    Package the Rust application and upload it to your Lambda function.

    ```sh
    cargo build --release --target x86_64-unknown-linux-musl
    zip -j lambda.zip target/x86_64-unknown-linux-musl/release/your_lambda_function
    aws lambda update-function-code --function-name your-lambda-function-name --zip-file fileb://lambda.zip
    ```

5. **Configure DynamoDB:**

    Ensure you have a DynamoDB table set up. You can use the AWS Management Console or AWS CLI to create a new table.

## Usage

Once the MediCompass application is deployed and configured, you can start using it to receive personalized medication recommendations:

1. **Invoke the Lambda function:**

    Use the AWS Management Console, AWS CLI, or any HTTP client to invoke your Lambda function with the required input parameters.

    ```sh
    aws lambda invoke --function-name your-lambda-function-name --payload '{ "patient_id": "12345" }' response.json
    ```

2. **Review the recommendations:**

    The Lambda function will process the input, connect to the MeldRX API to retrieve FHIR data, and return personalized medication recommendations based on the patient's health information.

## API Reference

For detailed information about the API endpoints and request/response formats, refer to the [API Reference Documentation](path/to/api-docs).

## Contributing

We welcome contributions to the MediCompass project! If you would like to contribute, please follow these steps:

1. Fork the repository.
2. Create a new branch: `git checkout -b feature-branch`.
3. Make your changes and commit them: `git commit -m 'Add new feature'`.
4. Push to the branch: `git push origin feature-branch`.
5. Submit a pull request.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

