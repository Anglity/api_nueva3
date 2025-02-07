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
                echo "Checking out from GitHub..."
                git branch: 'develop', url: 'https://github.com/Anglity/api_nueva3.git'
            }
        }
        stage('Build Docker Image') {
            steps {
                echo "Building Docker image..."
                sh "docker build -t $DOCKER_REGISTRY/$DOCKER_IMAGE:$DOCKER_TAG ."
            }
        }
        stage('Login to Nexus') {
            steps {
                echo "Logging into Nexus..."
                sh "echo 'Angel2610' | docker login -u admin --password-stdin http://$DOCKER_REGISTRY"
            }
        }
        stage('Push to Nexus') {
            steps {
                echo "Pushing image to Nexus..."
                sh "docker push $DOCKER_REGISTRY/$DOCKER_IMAGE:$DOCKER_TAG"
            }
        }
        stage('Deploy to Server') {
            steps {
                script {
                    def remote = [
                        name: 'targetServer',
                        host: SERVER_IP,
                        user: SERVER_USER,
                       identityFile: '/var/jenkins_home/.ssh/id_rsa',
                        allowAnyHosts: true
                    ]
                    sshCommand remote: remote, command: """
                        docker pull $DOCKER_REGISTRY/$DOCKER_IMAGE:$DOCKER_TAG
                        docker stop $DOCKER_IMAGE || true
                        docker rm $DOCKER_IMAGE || true
                        docker run -d -p 8080:8080 --name $DOCKER_IMAGE $DOCKER_REGISTRY/$DOCKER_IMAGE:$DOCKER_TAG
                    """
                }
            }
        }
    }
}
