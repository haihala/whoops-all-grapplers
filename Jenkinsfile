pipeline {
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