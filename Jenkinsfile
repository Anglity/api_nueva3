pipeline {
    agent any
    environment {
        DOCKER_REGISTRY = "167.71.164.51:8082"
        DOCKER_IMAGE = "api_nueva3"
        DOCKER_TAG = "latest"
        SERVER_USER = "root"
        SERVER_IP = "167.71.164.51"
        SSH_KEY_PATH = "/var/jenkins_home/.ssh/id_rsa"
        SSH_PASSPHRASE = "angelalvarez" // Passphrase de la clave privada
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
                    echo "Deploying application to server..."
                    sh '''
                    expect <<EOF
                    spawn ssh -o StrictHostKeyChecking=no -i $SSH_KEY_PATH $SERVER_USER@$SERVER_IP
                    expect "Enter passphrase for key"
                    send "$SSH_PASSPHRASE\r"
                    expect "$ "
                    send "docker pull $DOCKER_REGISTRY/$DOCKER_IMAGE:$DOCKER_TAG\r"
                    expect "$ "
                    send "docker stop $DOCKER_IMAGE || true\r"
                    expect "$ "
                    send "docker rm $DOCKER_IMAGE || true\r"
                    expect "$ "
                    send "docker run -d -p 8080:8080 --name $DOCKER_IMAGE $DOCKER_REGISTRY/$DOCKER_IMAGE:$DOCKER_TAG\r"
                    expect "$ "
                    send "exit\r"
                    EOF
                    '''
                }
            }
        }
    }
}
