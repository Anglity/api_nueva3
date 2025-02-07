pipeline {
    agent any
    environment {
        DOCKER_REGISTRY = "167.71.164.51:8082"
        DOCKER_IMAGE = "api_nueva3"
        DOCKER_TAG = "latest"
        SERVER_USER = "root"
        SERVER_IP = "167.71.164.51"
    }
    stages {
        stage('Checkout') {
            steps {
                git branch: 'develop', url: 'https://github.com/Anglity/api_nueva3.git'
            }
        }
        stage('Build Docker Image') {
            steps {
                sh "docker build -t $DOCKER_REGISTRY/$DOCKER_IMAGE:$DOCKER_TAG ."
            }
        }
        stage('Login to Nexus') {
            steps {
                sh "echo 'Angel2610' | docker login -u admin --password-stdin http://$DOCKER_REGISTRY"
            }
        }
        stage('Push to Nexus') {
            steps {
                sh "docker push $DOCKER_REGISTRY/$DOCKER_IMAGE:$DOCKER_TAG"
            }
        }
       stage('Deploy to Server') {
    steps {
        sshagent(credentials: ['ssh-server-credentials']) {
    sh """
    ssh -t root@167.71.164.51 <<EOF
    docker pull $DOCKER_REGISTRY/$DOCKER_IMAGE:$DOCKER_TAG
    docker stop $DOCKER_IMAGE || true
    docker rm $DOCKER_IMAGE || true
    docker run -d -p 8000:8000 --name $DOCKER_IMAGE $DOCKER_REGISTRY/$DOCKER_IMAGE:$DOCKER_TAG
    EOF
    """
}

    }
}

    }
}
