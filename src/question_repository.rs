use crate::models::Question;

/// Abstraction for loading questions (Open/Closed Principle & Dependency Inversion)
/// This trait allows extending with new implementations without modifying existing code
pub trait QuestionRepository {
    fn get_questions(&self) -> Vec<Question>;
}

/// In-memory implementation of QuestionRepository with hardcoded CKAD questions
pub struct InMemoryQuestionRepository;

impl QuestionRepository for InMemoryQuestionRepository {
    fn get_questions(&self) -> Vec<Question> {
        vec![
            Question {
                id: 1,
                question: "Create a Pod named 'nginx' using the nginx:1.14 image in the default namespace.".to_string(),
                hints: vec![
                    "Use: kubectl run <pod-name> --image=<image>".to_string(),
                    "Full command: kubectl run nginx --image=nginx:1.14".to_string(),
                    "Reference: https://kubernetes.io/docs/reference/kubectl/generated/kubectl-run/".to_string(),
                ],
                answer: "kubectl run nginx --image=nginx:1.14".to_string(),
                time_limit_secs: 60,
            },
            Question {
                id: 2,
                question: "Create a deployment named 'web' with 3 replicas using the httpd:2.4 image and expose port 80.".to_string(),
                hints: vec![
                    "Use kubectl create deployment, then kubectl set image, and kubectl expose".to_string(),
                    "Or use: kubectl create deployment web --image=httpd:2.4 --replicas=3".to_string(),
                    "Then: kubectl expose deployment web --port=80 --type=ClusterIP".to_string(),
                ],
                answer: "kubectl create deployment web --image=httpd:2.4 --replicas=3\nkubectl expose deployment web --port=80 --type=ClusterIP".to_string(),
                time_limit_secs: 120,
            },
            Question {
                id: 3,
                question: "Set resource requests and limits for a pod: request 256Mi memory and 100m CPU, limit 512Mi memory and 200m CPU.".to_string(),
                hints: vec![
                    "Use resources.requests and resources.limits in the pod spec".to_string(),
                    "Memory is specified in Mi, CPU in m (millicores)".to_string(),
                    "Reference: https://kubernetes.io/docs/concepts/configuration/manage-resources-containers/".to_string(),
                ],
                answer: "resources:\n  requests:\n    memory: \"256Mi\"\n    cpu: \"100m\"\n  limits:\n    memory: \"512Mi\"\n    cpu: \"200m\"".to_string(),
                time_limit_secs: 90,
            },
            Question {
                id: 4,
                question: "Create a ConfigMap named 'app-config' with key 'database.url' and value 'postgres://db:5432'.".to_string(),
                hints: vec![
                    "Use: kubectl create configmap <name> --from-literal=<key>=<value>".to_string(),
                    "Full command: kubectl create configmap app-config --from-literal=database.url=postgres://db:5432".to_string(),
                    "Reference: https://kubernetes.io/docs/concepts/configuration/configmap/".to_string(),
                ],
                answer: "kubectl create configmap app-config --from-literal=database.url=postgres://db:5432".to_string(),
                time_limit_secs: 60,
            },
            Question {
                id: 5,
                question: "Create a Secret named 'db-secret' with username 'admin' and password 'secret123'.".to_string(),
                hints: vec![
                    "Use: kubectl create secret generic <name> --from-literal=<key>=<value>".to_string(),
                    "Full command: kubectl create secret generic db-secret --from-literal=username=admin --from-literal=password=secret123".to_string(),
                    "Reference: https://kubernetes.io/docs/concepts/configuration/secret/".to_string(),
                ],
                answer: "kubectl create secret generic db-secret --from-literal=username=admin --from-literal=password=secret123".to_string(),
                time_limit_secs: 75,
            },
        ]
    }
}

/// Example: File-based implementation (extensible without modifying existing code)
/// This demonstrates the Open/Closed Principle - we can add new implementations
/// without modifying the QuestionRepository trait or InMemoryQuestionRepository
#[allow(dead_code)]
pub struct FileQuestionRepository {
    file_path: String,
}

#[allow(dead_code)]
impl FileQuestionRepository {
    pub fn new(file_path: String) -> Self {
        Self { file_path }
    }
}

#[allow(dead_code)]
impl QuestionRepository for FileQuestionRepository {
    fn get_questions(&self) -> Vec<Question> {
        // Future implementation: load from JSON/YAML file
        // This shows how the system is open for extension
        vec![]
    }
}
