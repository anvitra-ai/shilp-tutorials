# ECR Setup Instructions

Anvitra's enterprise Docker images are hosted on AWS Elastic Container Registry (ECR). This guide walks you through authenticating with ECR and pulling the images, even if you have never used AWS before.

---

## Prerequisites

- [Docker](https://docs.docker.com/get-docker/) installed and running
- AWS credentials provided by the Anvitra team:
  - `AWS_ACCESS_KEY_ID`
  - `AWS_SECRET_ACCESS_KEY`
  - `AWS_REGION` (e.g. `ap-south-1`)
  - ECR registry URL (e.g. `123456789012.dkr.ecr.ap-south-1.amazonaws.com`)

---

### Linux

```bash
curl "https://awscli.amazonaws.com/awscli-exe-linux-x86_64.zip" -o "awscliv2.zip"
unzip awscliv2.zip
sudo ./aws/install
```

---

## Step 2 — Configure AWS Credentials

Run the following command and enter the credentials provided by the Anvitra team when prompted:

```bash
aws configure
```

```
AWS Access Key ID [None]: <your-access-key-id>
AWS Secret Access Key [None]: <your-secret-access-key>
Default region name [None]: <your-region>         # e.g. ap-south-1
Default output format [None]: json
```

Alternatively, export them directly as environment variables (useful for CI or temporary sessions):

```bash
export AWS_ACCESS_KEY_ID=<your-access-key-id>
export AWS_SECRET_ACCESS_KEY=<your-secret-access-key>
export AWS_REGION=<your-region>
```

---

## Step 3 — Authenticate Docker with ECR

Use the AWS CLI to obtain a temporary ECR password and pass it directly to `docker login`:

```bash
aws ecr get-login-password --region <your-region> | \
  docker login --username AWS --password-stdin <your-ecr-registry-url>
```

**Example:**

```bash
aws ecr get-login-password --region ap-south-1 | \
  docker login --username AWS --password-stdin 123456789012.dkr.ecr.ap-south-1.amazonaws.com
```

You should see:

```
Login Succeeded
```

> **Note:** ECR login tokens are valid for **12 hours**. Re-run the command above if you get an authentication error while pulling images.

---

## Step 4 — Configure the Docker Compose File

Replace the `<replace-with-enterprise-repo>` placeholder in `docker-compose.yml` with the ECR registry URL provided by the Anvitra team:

```bash
./update-repo-name.sh <your-ecr-registry-url>
```

**Example:**

```bash
./update-repo-name.sh 123456789012.dkr.ecr.ap-south-1.amazonaws.com
```

---

## Step 5 — Set Up the Environment File

Copy the sample environment file and fill in the values provided by the Anvitra team:

```bash
cp sample.env .env
```

Open `.env` and update the following variables at minimum:

```dotenv
AWS_ACCESS_KEY_ID=<your-access-key-id>
AWS_SECRET_ACCESS_KEY=<your-secret-access-key>
AWS_REGION=<your-region>
```

---

## Step 6 — Start the Services

```bash
docker compose up -d
```

Docker will pull the images from ECR and start all services. To check that everything is running:

```bash
docker compose ps
```

---

## Troubleshooting

| Error                          | Cause                          | Fix                                                     |
| ------------------------------ | ------------------------------ | ------------------------------------------------------- |
| `no basic auth credentials`    | Docker not logged in to ECR    | Re-run the `docker login` command in Step 3             |
| `token has expired`            | ECR tokens last 12 hours       | Re-run the `aws ecr get-login-password` login command   |
| `Unable to locate credentials` | AWS CLI not configured         | Complete Step 2                                         |
| `permission denied`            | IAM user lacks ECR read access | Contact the Anvitra team to verify your IAM permissions |
