# IDS721 Mini Project 05

## Serverless Rust Microservice

## Getting started

### Setup Rust AWS Lambda function

1. Install the AWS CLI and configure it with your AWS credentials.

```
curl "https://awscli.amazonaws.com/awscli-exe-linux-x86_64.zip" -o "awscliv2.zip"
unzip awscliv2.zip
sudo ./aws/install
```
After the AWS CLI is installed, you can configure it with your AWS credentials using the `aws configure` command:

```
aws configure
```

This command will prompt you for your AWS Access Key ID, Secret Access Key, default region name, and default output format. You can find your Access Key ID and Secret Access Key in the AWS Management Console.

```
AWS Access Key ID [None]: YOUR_ACCESS_KEY
AWS Secret Access Key [None]: YOUR_SECRET_KEY
Default region name [None]: YOUR_DEFAULT_REGION
Default output format [None]: json
```


2. Install the Rust toolchain.
    -   Download and install rustup by running the following command in your terminal:

    ```
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
    ```

    -   The above command will download a script and start the installation. You will be prompted to proceed with the installation. Press "1" and hit Enter to proceed with the default installation.
    -   Once the installation is complete, close the terminal and open a new one.
    -   Add the cargo and rustc binaries to your PATH with this command:

    ```
    source $HOME/.cargo/env
    ```
    -   Verify that the installation was successful by running the following command:

    ```
    rustc --version
    ```
    -   You should see the version of the Rust compiler that was installed.

3. Create a new Rust project using `cargo new Project_Name`
```
cargo new week05
```


4. Add the lambda_runtime and serde_json dependencies in your Cargo.toml file.

```
lambda_runtime = "0.3.0"
serde_json = "1.0.64"
rusoto_dynamodb = "0.46.0"
rusoto_core = "0.46.0"
```

5. Write a handler function in main.rs that will be invoked by AWS Lambda.
    

6. Build your Rust project using cargo build --release.

```
cargo build --release
```

7. Package your function using the AWS CLI.

```
aws lambda create-function --function-name my-function --zip-file fileb://target/release/week05 --handler doesnt.matter --runtime provided --role arn:aws:iam::123456789012:role/execution_role
```


### Implement a simple service

1. Define the service interface, what it will do.



2. Implement the service logic in a separate Rust module.

3. Use the service in your AWS Lambda handler function.

### Connect to a database

1. Choose a database that suits your needs (e.g., Amazon RDS for a relational database).

2. Add the appropriate Rust database driver to your Cargo.toml file (e.g., diesel for PostgreSQL).

3. Implement database connection and query logic in your service.







## Add your files

- [ ] [Create](https://docs.gitlab.com/ee/user/project/repository/web_editor.html#create-a-file) or [upload](https://docs.gitlab.com/ee/user/project/repository/web_editor.html#upload-a-file) files
- [ ] [Add files using the command line](https://docs.gitlab.com/ee/gitlab-basics/add-file.html#add-a-file-using-the-command-line) or push an existing Git repository with the following command:

```
cd existing_repo
git remote add origin https://gitlab.com/dukeaiml/IDS721/ds655_ids721_miniproject05.git
git branch -M main
git push -uf origin main
```

