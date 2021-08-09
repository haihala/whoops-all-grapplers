pipeline {
    agent any

    stages {
        stage('build') {
            steps {
                sh 'cargo build'
            }
        }
        stage('test') {
            steps {
                sh 'cargo test'
            }
        }
    }
}