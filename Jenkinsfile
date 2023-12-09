pipeline {
  agent any
  stages {
    stage('Build & Test') {
      steps {
        sh "cargo make" 
      }
    }
  }
}

