pipeline {
    agent any

    environment {
        DOCKER_IMAGE = "angelalvarez0210/api_nueva"
        DOCKER_TAG = "latest"
        NEXUS_REPO = "http://209.97.159.2:8081/repository/docker-repo/"
        SERVER_IP = "209.97.159.2"
        SSH_KEY = credentials('ssh-key-id')
    }

    options {
        disableConcurrentBuilds() // Evita conflictos entre builds concurrentes
    }

    stages {
        stage('Checkout') {
            steps {
                git branch: "${env.BRANCH_NAME}", credentialsId: 'github-credentials', url: "https://github.com/Anglity/api_nueva3.git"
            }
        }

        stage('Build Docker Image') {
            when {
                not { branch 'main' } // Evita construir imágenes en main directamente
            }
            steps {
                sh """
                docker build -t ${DOCKER_IMAGE}:${DOCKER_TAG} .
                """
            }
        }

        stage('Push to Nexus') {
            when {
                not { branch 'main' } // Evita subir imágenes desde la rama main directamente
            }
            steps {
                sh """
                docker tag ${DOCKER_IMAGE}:${DOCKER_TAG} ${NEXUS_REPO}/${DOCKER_IMAGE}:${DOCKER_TAG}
                docker push ${NEXUS_REPO}/${DOCKER_IMAGE}:${DOCKER_TAG}
                """
            }
        }

        stage('Deploy to Server') {
            when {
                branch 'main' // Despliega solo cuando el cambio está en main
            }
            steps {
                sshagent(['ssh-key-id']) {
                    sh """
                    ssh root@${SERVER_IP} <<EOF
                    docker pull ${NEXUS_REPO}/${DOCKER_IMAGE}:${DOCKER_TAG}
                    docker stop api_nueva || true
                    docker run -d -p 8080:8080 --name api_nueva ${NEXUS_REPO}/${DOCKER_IMAGE}:${DOCKER_TAG}
                    EOF
                    """
                }
            }
        }
    }

    post {
        success {
            echo "✅ Despliegue exitoso en la rama main!"
        }
        failure {
            echo "❌ Error en el despliegue"
        }
    }
}
