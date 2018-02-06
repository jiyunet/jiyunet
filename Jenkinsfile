pipeline {
    agent any

    stages {
        stage('Build') {
            steps {
                sh 'cargo build --release'
            }
			archiveArtifacts artifacts: 'target/*/*', fingerprint: true
        }
        stage('Test') {
            steps {
                sh 'cargo test'
            }
        }
    }
}
