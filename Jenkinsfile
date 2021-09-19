pipeline {
    agent any

    stages {
        stage('Stable') {
            steps {
                sh "cargo build"
                sh "cargo test"
                sh "cargo clippy --all"
                sh "cargo fmt --all -- --check"
            }
        }

        stage('Nightly') {
            steps {
                sh "cargo +nightly build"
                sh "cargo +nightly test"
                sh "cargo +nightly clippy --all"
                sh "cargo +nightly fmt --all -- --check"
            }
        }
    }
}