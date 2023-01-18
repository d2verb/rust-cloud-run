# How to deploy to Cloud Run
1. push to Artifact Registry
```
$ gcloud artifacts repositories create rust-cloud-run --location=asia-northeast1 --repository-format=docker
$ gcloud auth configure-docker asia-northeast1-docker.pkg.dev
$ docker build . -t asia-northeast1-docker.pkg.dev/<PROJECT_ID>/rust-cloud-run/rust-cloud-run --target release
$ docker push asia-northeast1-docker.pkg.dev/<PROJECT_ID>/rust-cloud-run/rust-cloud-run
```
2. deploy to Cloud Run
```
$ gcloud run deploy rust-cloud-run --image asia-northeast1-docker.pkg.dev/<PROJECT_ID>/rust-cloud-run/rust-cloud-run
```
3. delete service
```
$ gcloud run services delete rust-cloud-run
```
