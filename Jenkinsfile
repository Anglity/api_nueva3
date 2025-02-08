pipeline {
    agent any
    environment {
        DOCKER_REGISTRY = "167.71.164.51:8082"
        DOCKER_IMAGE = "api_nueva"
        DOCKER_TAG = "latest"
        SERVER_USER = "root"
        SERVER_IP = "167.71.164.51"
        SSH_PASSPHRASE = "Angel2610" // Passphrase de la clave privada
        OLD_IMAGE_NAME = "angelalvarez0210/actix_backend-api_nueva:latest" // Imagen a detener
    }
    stages {
        stage('Checkout') {
            steps {
                echo "📥 Clonando código fuente desde GitHub..."
                git branch: 'develop', url: 'https://github.com/Anglity/api_nueva.git'
            }
        }
        stage('Build Docker Image') {
            steps {
                echo "🔨 Construyendo imagen Docker..."
                sh "docker build -t $DOCKER_REGISTRY/$DOCKER_IMAGE:$DOCKER_TAG ."
            }
        }
        stage('Login to Nexus') {
            steps {
                echo "🔑 Iniciando sesión en Nexus..."
                sh "echo '$SSH_PASSPHRASE' | docker login -u admin --password-stdin http://$DOCKER_REGISTRY"
            }
        }
        stage('Push to Nexus') {
            steps {
                echo "📤 Subiendo imagen a Nexus..."
                sh "docker push $DOCKER_REGISTRY/$DOCKER_IMAGE:$DOCKER_TAG"
            }
        }
        stage('Deploy to Server') {
            steps {
                echo "🚀 Desplegando aplicación en el servidor..."
                script {
                    sshagent(credentials: ['ssh-server-credentials']) {
                        sh """
                        ssh -o StrictHostKeyChecking=no -i /var/jenkins_home/.ssh/id_rsa $SERVER_USER@$SERVER_IP << 'ENDSSH'
                        echo "📥 Pulling la última imagen de Docker..."
                        docker pull $DOCKER_REGISTRY/$DOCKER_IMAGE:$DOCKER_TAG

                        echo "🔍 Verificando si existe un contenedor con la imagen antigua..."
                        OLD_CONTAINER_ID=\$(docker ps -q --filter "ancestor=$OLD_IMAGE_NAME")

                        if [ -n "\$OLD_CONTAINER_ID" ]; then
                            echo "🛑 Deteniendo el contenedor con la imagen antigua..."
                            docker stop \$OLD_CONTAINER_ID || true
                            echo "🗑️ Eliminando el contenedor con la imagen antigua..."
                            docker rm \$OLD_CONTAINER_ID || true
                        else
                            echo "✅ No se encontró ningún contenedor en ejecución con la imagen antigua."
                        fi

                        echo "🔍 Verificando si hay un contenedor existente con el nombre $DOCKER_IMAGE..."
                        EXISTING_CONTAINER=\$(docker ps -aq -f name=$DOCKER_IMAGE)

                        if [ -n "\$EXISTING_CONTAINER" ]; then
                            echo "🛑 Deteniendo el contenedor existente..."
                            docker stop \$EXISTING_CONTAINER || true
                            echo "🗑️ Eliminando el contenedor existente..."
                            docker rm \$EXISTING_CONTAINER || true
                        else
                            echo "✅ No se encontró ningún contenedor con el nombre $DOCKER_IMAGE."
                        fi

                        echo "🏃‍♂️ Iniciando un nuevo contenedor..."
                        docker run -d --restart unless-stopped --name $DOCKER_IMAGE -p 8080:8080 $DOCKER_REGISTRY/$DOCKER_IMAGE:$DOCKER_TAG

                        echo "✅ Despliegue completado exitosamente!"
                        exit
                        ENDSSH
                        """
                    }
                }
            }
        }
    }
    post {
        success {
            echo "🎉 Pipeline completado exitosamente!"
        }
        failure {
            echo "🚨 ERROR: Algo falló en el pipeline, revisa los logs!"
        }
    }
}
